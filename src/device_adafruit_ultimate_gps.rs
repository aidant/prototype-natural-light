use defmt::*;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    peripherals,
    usart::{self, Config, Uart},
};
use heapless::Vec;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub async fn get_messages(
    usart: peripherals::USART1,
    pin_rx: peripherals::PB7,
    pin_tx: peripherals::PB6,
    dma_rx: peripherals::DMA2_CH2,
    dma_tx: NoDma,
) {
    info!("Get GPS Coordinates");

    let mut config = Config::default();

    config.baudrate = 9600;

    let mut usart = Uart::new(usart, pin_rx, pin_tx, Irqs, dma_tx, dma_rx, config);

    let mut message = Vec::<u8, 512>::new();

    let mut buf = [0u8; 1];
    loop {
        usart.read(&mut buf).await.unwrap();

        for byte in buf {
            message.push(byte).unwrap();

            if byte == b'\n' {
                let sentence = core::str::from_utf8(&message).unwrap();

                info!("{}", sentence);

                message.clear();
            }
        }
    }
}
