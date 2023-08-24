use defmt::*;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, UartRx},
    Peripherals,
};
use heapless::Vec;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub async fn get_messages(p: Peripherals) {
    let mut config = Config::default();
    config.baudrate = 9600;

    let mut dma_buf = [0u8; 256];

    let mut usart =
        UartRx::new(p.USART1, Irqs, p.PB7, p.DMA2_CH2, config).into_ring_buffered(&mut dma_buf);

    let mut message = Vec::<u8, 128>::new();
    let mut buf = [0u8; 64];
    loop {
        let len = usart.read(&mut buf).await.unwrap();

        for byte in buf.iter().take(len) {
            message.push(*byte).unwrap();

            if *byte == b'\n' {
                let sentence = core::str::from_utf8(&message).unwrap();

                info!("{}", sentence);

                message.clear();
            }
        }
    }
}
