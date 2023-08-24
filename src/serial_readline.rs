use embassy_stm32::usart::{BasicInstance, RingBufferedUartRx, RxDma, UartRx};
use futures_core::Stream;

pub struct SerialReadline<'a, T, Dma>
where
    T: BasicInstance,
    Dma: RxDma<T>,
{
    uart: RingBufferedUartRx<'a, T, Dma>,
}

impl<'a, T, Dma> SerialReadline<'a, T, Dma>
where
    T: BasicInstance,
    Dma: RxDma<T>,
{
    pub fn new(uart: UartRx<'a, T, Dma>) -> Self {
        let mut dma_buf = [0u8; 1];

        Self {
            uart: uart.into_ring_buffered(&mut dma_buf),
        }
    }
}
