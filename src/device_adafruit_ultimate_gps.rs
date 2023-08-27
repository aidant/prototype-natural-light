use defmt::Format;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, RingBufferedUartRx, UartRx},
};
use heapless::{String, Vec};
use static_cell::StaticCell;

static DMA_BUF: StaticCell<[u8; 256]> = StaticCell::new();

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub struct AdafruitUltimateGps<'a> {
    usart: RingBufferedUartRx<'a, peripherals::USART1, peripherals::DMA2_CH2>,
    buf: [u8; 64],
    msg: Vec<u8, 96>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum Error {
    SerialFraming,
    SerialNoise,
    SerialOverrun,
    SerialParity,
    SerialBufferTooLong,
    SerialOther,

    MessageOverrun,

    MessageUtf8Error,
}

impl From<usart::Error> for Error {
    fn from(value: usart::Error) -> Self {
        match value {
            usart::Error::Framing => Self::SerialFraming,
            usart::Error::Noise => Self::SerialNoise,
            usart::Error::Overrun => Self::SerialOverrun,
            usart::Error::Parity => Self::SerialParity,
            usart::Error::BufferTooLong => Self::SerialBufferTooLong,
            _ => Self::SerialOther,
        }
    }
}

impl From<u8> for Error {
    fn from(_value: u8) -> Self {
        Self::MessageOverrun
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_value: core::str::Utf8Error) -> Self {
        Self::MessageUtf8Error
    }
}

impl<'a> AdafruitUltimateGps<'a> {
    pub fn new(
        peri: peripherals::USART1,
        rx_pin: peripherals::PB7,
        rx_dma: peripherals::DMA2_CH2,
    ) -> Self {
        let mut config = Config::default();
        config.baudrate = 9600;

        let dma_buf = DMA_BUF.init([0u8; 256]);

        let usart = UartRx::new(peri, Irqs, rx_pin, rx_dma, config).into_ring_buffered(dma_buf);

        Self {
            usart,
            buf: [0u8; 64],
            msg: Vec::<u8, 96>::new(),
        }
    }

    pub async fn read_message(&mut self) -> Result<String<96>, Error> {
        let mut string = Option::<String<96>>::None;

        loop {
            let len = self.usart.read(&mut self.buf).await?;

            for byte in self.buf.iter().take(len) {
                self.msg.push(*byte)?;

                if *byte == b'\n' {
                    string = Option::Some(String::from(core::str::from_utf8(&self.msg)?));

                    self.msg.clear();
                }
            }

            if let Option::Some(message) = string {
                return Result::Ok(message);
            }
        }
    }
}
