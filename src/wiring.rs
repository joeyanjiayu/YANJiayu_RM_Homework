//! Complete the two TODOs in this file.

use embassy_stm32::{Peripherals, can::Can, usart::BufferedUart};

use crate::swd::Swd;

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: Pick two exposed 3.3 V GPIOs and pass them to Swd::new.
    // Use Output for SWCLK (initially low), and Flex for bidirectional SWDIO.
    let swclk = embassy_stm32::gpio::Output::new(p.PA0,embassy_stm32::gpio::Level::Low,embassy_stm32::gpio::Speed::Medium,);
    let swdio = embassy_stm32::gpio::Flex::new(p.PA2);
    let swd = Some(Swd::new(swclk, swdio));

    // TODO 2: Configure a buffered UART at 115200 baud, 8N1.
    // Identify the peripheral, RX/TX pins and interrupt from the schematic.
    // Bind the interrupt and provide static RX and TX buffers.
    embassy_stm32::bind_interrupts!(struct Irqs {UART7 => embassy_stm32::usart::BufferedInterruptHandler<embassy_stm32::peripherals::UART7>;});

    static TX_BUF: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static RX_BUF: static_cell::StaticCell<[u8; 1024]> = static_cell::StaticCell::new();

    let mut config = embassy_stm32::usart::Config::default();
    config.baudrate = 115_200;

    config.eager_reads = Some(1);

    let uart = Some(BufferedUart::new(p.UART7,p.PE7,p.PE8,TX_BUF.init([0; 256]),RX_BUF.init([0; 1024]),Irqs,config,).unwrap(),);

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart }, swd)
}
