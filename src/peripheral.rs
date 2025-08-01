#![no_main]
#![no_std]

#[macro_use]
mod macros;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Output},
    peripherals::{PIO0, USB},
    usb::InterruptHandler,
};
use panic_halt as _;
use rmk::{
    channel::EVENT_CHANNEL,
    debounce::fast_debouncer::RapidDebouncer,
    futures::future::join,
    matrix::Matrix,
    run_devices,
    split::{
        peripheral::run_rmk_split_peripheral,
        rp::uart::{BufferedUart, UartInterruptHandler},
        SPLIT_MESSAGE_MAX_SIZE,
    },
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    PIO0_IRQ_0 => UartInterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Pin config

    let (input_pins, output_pins) = config_matrix_pins_rp!(peripherals: p, input: [PIN_7, PIN_8, PIN_9, PIN_10], output: [PIN_16, PIN_15, PIN_14, PIN_13, PIN_12]);

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance =
        BufferedUart::new_full_duplex(p.PIO0, p.PIN_1, p.PIN_0, tx_buf, rx_buf, Irqs);

    // Define the matrix
    let debouncer = RapidDebouncer::<4, 5>::new();
    let mut matrix = Matrix::<_, _, _, 4, 5>::new(input_pins, output_pins, debouncer);

    // Start
    join(
        run_devices!((matrix) => EVENT_CHANNEL), // Peripheral uses EVENT_CHANNEL to send events to central
        run_rmk_split_peripheral(uart_instance),
    )
    .await;
}
