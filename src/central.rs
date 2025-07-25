#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
mod vial;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Output},
    peripherals::{PIO0, USB},
    usb::{Driver, InterruptHandler},
};
use panic_halt as _;
use rmk::{
    channel::EVENT_CHANNEL,
    config::{
        BehaviorConfig, ControllerConfig, KeyboardUsbConfig, RmkConfig, TapHoldConfig, TapHoldMode,
        VialConfig,
    },
    debounce::fast_debouncer::RapidDebouncer,
    futures::future::join4,
    initialize_keymap,
    input_device::Runnable,
    keyboard::Keyboard,
    light::LightController,
    run_devices, run_rmk,
    split::{
        central::{run_peripheral_manager, CentralMatrix},
        rp::uart::{BufferedUart, UartInterruptHandler},
        SPLIT_MESSAGE_MAX_SIZE,
    },
};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    PIO0_IRQ_0 => UartInterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    let (input_pins, output_pins) = config_matrix_pins_rp!(peripherals: p, input: [PIN_7, PIN_8, PIN_9, PIN_10], output: [PIN_12, PIN_13, PIN_14, PIN_15, PIN_16]);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0xbeeb,
        pid: 0x0002,
        manufacturer: "beekeeb",
        product_name: "piantor-rmk",
        serial_number: "vial:f64c2b3c:000001",
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);

    let rmk_config = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        ..Default::default()
    };

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_receiver =
        BufferedUart::new_full_duplex(p.PIO0, p.PIN_0, p.PIN_1, tx_buf, rx_buf, Irqs);

    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let mut tap_hold_config = TapHoldConfig::default();
    tap_hold_config.enable_hrm = true;
    // tap_hold_config.chordal_hold = true;
    tap_hold_config.mode = TapHoldMode::PermissiveHold;
    let mut behavior_config = BehaviorConfig::default();
    behavior_config.tap_hold = tap_hold_config;
    behavior_config.combo = keymap::get_combo_config();
    let keymap = initialize_keymap(&mut default_keymap, behavior_config).await;

    // Initialize the matrix + keyboard
    let debouncer = RapidDebouncer::<4, 5>::new();
    let mut matrix = CentralMatrix::<_, _, _, 0, 0, 4, 5>::new(input_pins, output_pins, debouncer);
    let mut keyboard = Keyboard::new(&keymap);

    // Initialize the light controller
    let mut light_controller: LightController<Output> =
        LightController::new(ControllerConfig::default().light_config);

    // Start
    join4(
        run_devices! (
            (matrix) => EVENT_CHANNEL,
        ),
        keyboard.run(),
        run_peripheral_manager::<4, 5, 0, 5, _>(0, uart_receiver),
        run_rmk(&keymap, driver, &mut light_controller, rmk_config),
    )
    .await;
}
