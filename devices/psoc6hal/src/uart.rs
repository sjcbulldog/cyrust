use super::scb::hw::* ;

pub struct Uart {
    pub port: u8,
}

//     .uartMode                   = CY_SCB_UART_STANDARD,
//     .oversample                 = 12UL,
//     .enableMsbFirst             = false,
//     .dataWidth                  = 8UL,
//     .parity                     = CY_SCB_UART_PARITY_NONE,
//     .stopBits                   = CY_SCB_UART_STOP_BITS_1,
//     .enableInputFilter          = false,
//     .breakWidth                 = 11UL,


impl Uart {
    pub fn init(&mut self, bits: u8, stop: u8, over: u8) -> u32 {
        let scbhw = get_hw(self.port) ;

            scbhw.ctrl.write(|w| unsafe { w.ovs().bits(over - 1).byte_mode().set_bit().mode().bits(2).enabled().clear_bit()  }) ;
            let scb_ctrl = scbhw.ctrl.read().bits() ;

            scbhw.uart_ctrl.write(|w| unsafe { w.mode().bits(0)}) ;
            let scb_uart_ctrl = scbhw.uart_ctrl.read().bits() ;

            scbhw.uart_rx_ctrl.write(|w| unsafe { w.break_width().bits(10).stop_bits().bits(stop) }) ;
            let scb_uart_rx_ctrl = scbhw.uart_rx_ctrl.read().bits() ;

            scbhw.rx_ctrl.write(|w| unsafe { w.data_width().bits(bits - 1).msb_first().clear_bit() }) ;
            let scb_rx_ctrl = scbhw.rx_ctrl.read().bits() ;

            scbhw.uart_tx_ctrl.write(|w| unsafe { w.stop_bits().bits(stop) }) ;
            let scb_uart_tx_ctrl = scbhw.uart_tx_ctrl.read().bits() ;

            scbhw.tx_ctrl.write(|w| unsafe { w.data_width().bits(bits - 1).msb_first().clear_bit()}) ;
            let scb_tx_ctrl = scbhw.tx_ctrl.read().bits() ;

            return scb_ctrl + scb_uart_ctrl + scb_uart_rx_ctrl + scb_rx_ctrl + scb_uart_tx_ctrl + scb_tx_ctrl ;
    }

    pub fn enable(&mut self) {
        let scbhw = get_hw(self.port) ;
        scbhw.ctrl.modify(|_r, w| { w.enabled().set_bit() }) ;
    }

    pub fn put_char(&self, ch: u8) {
        // Wait for the fifo to empty
        while self.get_num_in_tx_fifo() != 0 {
        }

        self.write_tx_fifo(ch as u16) ;
    }

    pub fn put_string(&self, s: &str) {
        for ch in s.chars() {
            self.put_char(ch as u8) ;
        }
    }

    pub fn get_num_in_tx_fifo(&self) -> u16 {
        let scbhw = get_hw(self.port) ;

        let st = scbhw.tx_fifo_status.read() ;
        return st.used().bits() ;
    }

    pub fn write_tx_fifo(&self, ch: u16) {
        let scbhw = get_hw(self.port) ;

        scbhw.tx_fifo_wr.write(|w| unsafe { w.data().bits(ch) }) ;
    }
}
