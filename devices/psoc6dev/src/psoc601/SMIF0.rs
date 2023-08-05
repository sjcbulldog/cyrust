#![allow(non_camel_case_types)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0621765 2023-07-02))"]
#[doc = "Device (only used in XIP mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Device {
    ptr: *mut u8,
}
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
impl Device {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<DeviceCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Device region base address"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Device region mask"]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Address control"]
    #[inline(always)]
    pub const fn addr_ctl(self) -> crate::common::Reg<AddrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Read command control"]
    #[inline(always)]
    pub const fn rd_cmd_ctl(self) -> crate::common::Reg<RdCmdCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Read address control"]
    #[inline(always)]
    pub const fn rd_addr_ctl(self) -> crate::common::Reg<RdAddrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Read mode control"]
    #[inline(always)]
    pub const fn rd_mode_ctl(self) -> crate::common::Reg<RdModeCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Read dummy control"]
    #[inline(always)]
    pub const fn rd_dummy_ctl(self) -> crate::common::Reg<RdDummyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Read data control"]
    #[inline(always)]
    pub const fn rd_data_ctl(self) -> crate::common::Reg<RdDataCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Write command control"]
    #[inline(always)]
    pub const fn wr_cmd_ctl(self) -> crate::common::Reg<WrCmdCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Write address control"]
    #[inline(always)]
    pub const fn wr_addr_ctl(self) -> crate::common::Reg<WrAddrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Write mode control"]
    #[inline(always)]
    pub const fn wr_mode_ctl(self) -> crate::common::Reg<WrModeCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Write dummy control"]
    #[inline(always)]
    pub const fn wr_dummy_ctl(self) -> crate::common::Reg<WrDummyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Write data control"]
    #[inline(always)]
    pub const fn wr_data_ctl(self) -> crate::common::Reg<WrDataCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
}
#[doc = "Serial Memory Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smif0 {
    ptr: *mut u8,
}
unsafe impl Send for Smif0 {}
unsafe impl Sync for Smif0 {}
impl Smif0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<Smif0Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Transmitter command FIFO status"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_status(self) -> crate::common::Reg<TxCmdFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Transmitter command FIFO write"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_wr(self) -> crate::common::Reg<TxCmdFifoWr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Transmitter data FIFO control"]
    #[inline(always)]
    pub const fn tx_data_fifo_ctl(self) -> crate::common::Reg<TxDataFifoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Transmitter data FIFO status"]
    #[inline(always)]
    pub const fn tx_data_fifo_status(
        self,
    ) -> crate::common::Reg<TxDataFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr1(self) -> crate::common::Reg<TxDataFifoWr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr2(self) -> crate::common::Reg<TxDataFifoWr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr4(self) -> crate::common::Reg<TxDataFifoWr4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "Receiver data FIFO control"]
    #[inline(always)]
    pub const fn rx_data_fifo_ctl(self) -> crate::common::Reg<RxDataFifoCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Receiver data FIFO status"]
    #[inline(always)]
    pub const fn rx_data_fifo_status(
        self,
    ) -> crate::common::Reg<RxDataFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1(self) -> crate::common::Reg<RxDataFifoRd1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd2(self) -> crate::common::Reg<RxDataFifoRd2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd4(self) -> crate::common::Reg<RxDataFifoRd4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "Receiver data FIFO silent read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1_silent(
        self,
    ) -> crate::common::Reg<RxDataFifoRd1Silent, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Slow cache control"]
    #[inline(always)]
    pub const fn slow_ca_ctl(self) -> crate::common::Reg<SlowCaCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Slow cache command"]
    #[inline(always)]
    pub const fn slow_ca_cmd(self) -> crate::common::Reg<SlowCaCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Fast cache control"]
    #[inline(always)]
    pub const fn fast_ca_ctl(self) -> crate::common::Reg<FastCaCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Fast cache command"]
    #[inline(always)]
    pub const fn fast_ca_cmd(self) -> crate::common::Reg<FastCaCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "Cryptography Command"]
    #[inline(always)]
    pub const fn crypto_cmd(self) -> crate::common::Reg<CryptoCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Cryptography input 0"]
    #[inline(always)]
    pub const fn crypto_input0(self) -> crate::common::Reg<CryptoInput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Cryptography input 1"]
    #[inline(always)]
    pub const fn crypto_input1(self) -> crate::common::Reg<CryptoInput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(548usize) as _) }
    }
    #[doc = "Cryptography input 2"]
    #[inline(always)]
    pub const fn crypto_input2(self) -> crate::common::Reg<CryptoInput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(552usize) as _) }
    }
    #[doc = "Cryptography input 3"]
    #[inline(always)]
    pub const fn crypto_input3(self) -> crate::common::Reg<CryptoInput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(556usize) as _) }
    }
    #[doc = "Cryptography key 0"]
    #[inline(always)]
    pub const fn crypto_key0(self) -> crate::common::Reg<CryptoKey0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Cryptography key 1"]
    #[inline(always)]
    pub const fn crypto_key1(self) -> crate::common::Reg<CryptoKey1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(580usize) as _) }
    }
    #[doc = "Cryptography key 2"]
    #[inline(always)]
    pub const fn crypto_key2(self) -> crate::common::Reg<CryptoKey2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(584usize) as _) }
    }
    #[doc = "Cryptography key 3"]
    #[inline(always)]
    pub const fn crypto_key3(self) -> crate::common::Reg<CryptoKey3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(588usize) as _) }
    }
    #[doc = "Cryptography output 0"]
    #[inline(always)]
    pub const fn crypto_output0(self) -> crate::common::Reg<CryptoOutput0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(608usize) as _) }
    }
    #[doc = "Cryptography output 1"]
    #[inline(always)]
    pub const fn crypto_output1(self) -> crate::common::Reg<CryptoOutput1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(612usize) as _) }
    }
    #[doc = "Cryptography output 2"]
    #[inline(always)]
    pub const fn crypto_output2(self) -> crate::common::Reg<CryptoOutput2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(616usize) as _) }
    }
    #[doc = "Cryptography output 3"]
    #[inline(always)]
    pub const fn crypto_output3(self) -> crate::common::Reg<CryptoOutput3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(620usize) as _) }
    }
    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1984usize) as _) }
    }
    #[doc = "Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1988usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1992usize) as _) }
    }
    #[doc = "Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1996usize) as _) }
    }
    #[doc = "Device (only used in XIP mode)"]
    #[inline(always)]
    pub const fn device(self, n: usize) -> Device {
        assert!(n < 4usize);
        unsafe { Device::from_ptr(self.ptr.add(2048usize + n * 128usize) as _) }
    }
}
#[doc = "Device region base address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u32);
impl Addr {
    #[doc = "Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\] = SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\] = SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
#[doc = "Address control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AddrCtl(pub u32);
impl AddrCtl {
    #[doc = "Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub const fn size2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn set_size2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub const fn div2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn set_div2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for AddrCtl {
    #[inline(always)]
    fn default() -> AddrCtl {
        AddrCtl(0)
    }
}
#[doc = "Cryptography Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoCmd(pub u32);
impl CryptoCmd {
    #[doc = "SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SW sets this field to '1' to start a AES-128 forward block cipher operation (on the address in CRYPTO_ADDR). HW sets this field to '0' to indicate that the operation has completed. Once completed, the result of the operation can be read from CRYPTO_RESULT0, ..., CRYPTO_RESULT3. The operation takes roughly 13 clk_hf clock cycles. Note: An operation can only be started in MMIO_MODE."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CryptoCmd {
    #[inline(always)]
    fn default() -> CryptoCmd {
        CryptoCmd(0)
    }
}
#[doc = "Cryptography input 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoInput0(pub u32);
impl CryptoInput0 {
    #[doc = "Four Bytes of the plaintext PT\\[31:0\\] = CRYPTO_INPUT0.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn input(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the plaintext PT\\[31:0\\] = CRYPTO_INPUT0.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_input(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoInput0 {
    #[inline(always)]
    fn default() -> CryptoInput0 {
        CryptoInput0(0)
    }
}
#[doc = "Cryptography input 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoInput1(pub u32);
impl CryptoInput1 {
    #[doc = "Four Bytes of the plaintext PT\\[63:32\\] = CRYPTO_INPUT1.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn input(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the plaintext PT\\[63:32\\] = CRYPTO_INPUT1.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_input(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoInput1 {
    #[inline(always)]
    fn default() -> CryptoInput1 {
        CryptoInput1(0)
    }
}
#[doc = "Cryptography input 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoInput2(pub u32);
impl CryptoInput2 {
    #[doc = "Four Bytes of the plaintext PT\\[95:64\\] = CRYPTO_INPUT2.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn input(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the plaintext PT\\[95:64\\] = CRYPTO_INPUT2.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_input(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoInput2 {
    #[inline(always)]
    fn default() -> CryptoInput2 {
        CryptoInput2(0)
    }
}
#[doc = "Cryptography input 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoInput3(pub u32);
impl CryptoInput3 {
    #[doc = "Four Bytes of the plaintext PT\\[127:96\\] = CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn input(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the plaintext PT\\[127:96\\] = CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_input(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoInput3 {
    #[inline(always)]
    fn default() -> CryptoInput3 {
        CryptoInput3(0)
    }
}
#[doc = "Cryptography key 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKey0(pub u32);
impl CryptoKey0 {
    #[doc = "Four Bytes of the key KEY\\[31:0\\] = CRYPTO_KEY0.KEY\\[31:0\\]."]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the key KEY\\[31:0\\] = CRYPTO_KEY0.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoKey0 {
    #[inline(always)]
    fn default() -> CryptoKey0 {
        CryptoKey0(0)
    }
}
#[doc = "Cryptography key 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKey1(pub u32);
impl CryptoKey1 {
    #[doc = "Four Bytes of the key KEY\\[63:32\\] = CRYPTO_KEY1.KEY\\[31:0\\]."]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the key KEY\\[63:32\\] = CRYPTO_KEY1.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoKey1 {
    #[inline(always)]
    fn default() -> CryptoKey1 {
        CryptoKey1(0)
    }
}
#[doc = "Cryptography key 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKey2(pub u32);
impl CryptoKey2 {
    #[doc = "Four Bytes of the key KEY\\[95:64\\] = CRYPTO_KEY2.KEY\\[31:0\\]."]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the key KEY\\[95:64\\] = CRYPTO_KEY2.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoKey2 {
    #[inline(always)]
    fn default() -> CryptoKey2 {
        CryptoKey2(0)
    }
}
#[doc = "Cryptography key 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKey3(pub u32);
impl CryptoKey3 {
    #[doc = "Four Bytes of the key KEY\\[127:96\\] = CRYPTO_KEY3.KEY\\[31:0\\]."]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the key KEY\\[127:96\\] = CRYPTO_KEY3.KEY\\[31:0\\]."]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoKey3 {
    #[inline(always)]
    fn default() -> CryptoKey3 {
        CryptoKey3(0)
    }
}
#[doc = "Cryptography output 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoOutput0(pub u32);
impl CryptoOutput0 {
    #[doc = "Four Bytes of the ciphertext CT\\[31:0\\] = CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn output(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the ciphertext CT\\[31:0\\] = CRYPTO_OUTPUT0.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_output(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoOutput0 {
    #[inline(always)]
    fn default() -> CryptoOutput0 {
        CryptoOutput0(0)
    }
}
#[doc = "Cryptography output 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoOutput1(pub u32);
impl CryptoOutput1 {
    #[doc = "Four Bytes of the ciphertext CT\\[63:32\\] = CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn output(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the ciphertext CT\\[63:32\\] = CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_output(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoOutput1 {
    #[inline(always)]
    fn default() -> CryptoOutput1 {
        CryptoOutput1(0)
    }
}
#[doc = "Cryptography output 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoOutput2(pub u32);
impl CryptoOutput2 {
    #[doc = "Four Bytes of the ciphertext CT\\[95:64\\] = CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn output(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the ciphertext CT\\[95:64\\] = CRYPTO_OUTPUT2.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_output(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoOutput2 {
    #[inline(always)]
    fn default() -> CryptoOutput2 {
        CryptoOutput2(0)
    }
}
#[doc = "Cryptography output 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoOutput3(pub u32);
impl CryptoOutput3 {
    #[doc = "Four Bytes of the ciphertext CT\\[127:96\\] = CRYPTO_OUTPUT3.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub const fn output(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Four Bytes of the ciphertext CT\\[127:96\\] = CRYPTO_OUTPUT3.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn set_output(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CryptoOutput3 {
    #[inline(always)]
    fn default() -> CryptoOutput3 {
        CryptoOutput3(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceCtl(pub u32);
impl DeviceCtl {
    #[doc = "Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub const fn wr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn set_wr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub const fn crypto_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn set_crypto_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\] = IO0, spi_data\\[1\\] = IO1, ..., spi_data\\[7\\] = IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\] = IO0, spi_data\\[3\\] = IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\] = IO0, spi_data\\[5\\] = IO1, ..., spi_data\\[7\\] = IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\] = IO0, spi_data\\[7\\] = IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub const fn data_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\] = IO0, spi_data\\[1\\] = IO1, ..., spi_data\\[7\\] = IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\] = IO0, spi_data\\[3\\] = IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\] = IO0, spi_data\\[5\\] = IO1, ..., spi_data\\[7\\] = IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\] = IO0, spi_data\\[7\\] = IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn set_data_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DeviceCtl {
    #[inline(always)]
    fn default() -> DeviceCtl {
        DeviceCtl(0)
    }
}
#[doc = "Fast cache command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FastCaCmd(pub u32);
impl FastCaCmd {
    #[doc = "See SLOW_CA_CMD.INV."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See SLOW_CA_CMD.INV."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for FastCaCmd {
    #[inline(always)]
    fn default() -> FastCaCmd {
        FastCaCmd(0)
    }
}
#[doc = "Fast cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FastCaCtl(pub u32);
impl FastCaCtl {
    #[doc = "See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    pub const fn way(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    pub fn set_way(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub const fn set_addr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FastCaCtl {
    #[inline(always)]
    fn default() -> FastCaCtl {
        FastCaCtl(0)
    }
}
#[doc = "Interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub const fn tr_tx_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn set_tr_tx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub const fn tr_rx_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn set_tr_rx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub const fn xip_alignment_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn set_xip_alignment_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub const fn tx_cmd_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn set_tx_cmd_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub const fn tx_data_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn set_tx_data_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub const fn rx_data_fifo_underflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn set_rx_data_fifo_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tr_tx_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tr_tx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tr_rx_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tr_rx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn xip_alignment_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_xip_alignment_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_cmd_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_cmd_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_data_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_data_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_data_fifo_underflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_data_fifo_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tr_tx_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tr_tx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tr_rx_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tr_rx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn xip_alignment_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_xip_alignment_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_cmd_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_cmd_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn tx_data_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tx_data_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn rx_data_fifo_underflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_rx_data_fifo_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tr_tx_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tr_tx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tr_rx_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tr_rx_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn xip_alignment_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_xip_alignment_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_cmd_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_cmd_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tx_data_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tx_data_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn rx_data_fifo_underflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_rx_data_fifo_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Device region mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc = "Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\] with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\] & MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\] = 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Specifies the size of the device region. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\] with the address as specified in ADDR.ADDR: Address A is in the device when (A\\[31:8\\] & MASK\\[31:8\\]) == ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\] = 0xff. Note: a transfer request that is not in any device region results in an AHB-Lite bus error."]
    #[inline(always)]
    pub fn set_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
#[doc = "Read address control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdAddrCtl(pub u32);
impl RdAddrCtl {
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for RdAddrCtl {
    #[inline(always)]
    fn default() -> RdAddrCtl {
        RdAddrCtl(0)
    }
}
#[doc = "Read command control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdCmdCtl(pub u32);
impl RdCmdCtl {
    #[doc = "Command byte code."]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command byte code."]
    #[inline(always)]
    pub fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RdCmdCtl {
    #[inline(always)]
    fn default() -> RdCmdCtl {
        RdCmdCtl(0)
    }
}
#[doc = "Read data control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdDataCtl(pub u32);
impl RdDataCtl {
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for RdDataCtl {
    #[inline(always)]
    fn default() -> RdDataCtl {
        RdDataCtl(0)
    }
}
#[doc = "Read dummy control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdDummyCtl(pub u32);
impl RdDummyCtl {
    #[doc = "Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub const fn size5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn set_size5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RdDummyCtl {
    #[inline(always)]
    fn default() -> RdDummyCtl {
        RdDummyCtl(0)
    }
}
#[doc = "Read mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdModeCtl(pub u32);
impl RdModeCtl {
    #[doc = "Mode byte code."]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Mode byte code."]
    #[inline(always)]
    pub fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RdModeCtl {
    #[inline(always)]
    fn default() -> RdModeCtl {
        RdModeCtl(0)
    }
}
#[doc = "Receiver data FIFO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoCtl(pub u32);
impl RxDataFifoCtl {
    #[doc = "Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RxDataFifoCtl {
    #[inline(always)]
    fn default() -> RxDataFifoCtl {
        RxDataFifoCtl(0)
    }
}
#[doc = "Receiver data FIFO read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoRd1(pub u32);
impl RxDataFifoRd1 {
    #[doc = "RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RxDataFifoRd1 {
    #[inline(always)]
    fn default() -> RxDataFifoRd1 {
        RxDataFifoRd1(0)
    }
}
#[doc = "Receiver data FIFO silent read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoRd1Silent(pub u32);
impl RxDataFifoRd1Silent {
    #[doc = "RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RxDataFifoRd1Silent {
    #[inline(always)]
    fn default() -> RxDataFifoRd1Silent {
        RxDataFifoRd1Silent(0)
    }
}
#[doc = "Receiver data FIFO read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoRd2(pub u32);
impl RxDataFifoRd2 {
    #[doc = "RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for RxDataFifoRd2 {
    #[inline(always)]
    fn default() -> RxDataFifoRd2 {
        RxDataFifoRd2(0)
    }
}
#[doc = "Receiver data FIFO read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoRd4(pub u32);
impl RxDataFifoRd4 {
    #[doc = "RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "RX data (read from RX data FIFO, third byte)."]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, third byte)."]
    #[inline(always)]
    pub fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "RX data (read from RX data FIFO, fourth byte)."]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "RX data (read from RX data FIFO, fourth byte)."]
    #[inline(always)]
    pub fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for RxDataFifoRd4 {
    #[inline(always)]
    fn default() -> RxDataFifoRd4 {
        RxDataFifoRd4(0)
    }
}
#[doc = "Receiver data FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDataFifoStatus(pub u32);
impl RxDataFifoStatus {
    #[doc = "Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub const fn used4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn set_used4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for RxDataFifoStatus {
    #[inline(always)]
    fn default() -> RxDataFifoStatus {
        RxDataFifoStatus(0)
    }
}
#[doc = "Slow cache command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlowCaCmd(pub u32);
impl SlowCaCmd {
    #[doc = "Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cache and prefetch buffer invalidation. SW writes a '1' to clear the cache and prefetch buffer. The cache's LRU structure is also reset to its default state. Note, A write access will invalidate the prefetch buffer automatically in hardware. A write access should invalidate both fast and slow caches, by firmware. Note, firmware should invalidate the cache and prefetch buffer only when STATUS.BUSY is '0'."]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SlowCaCmd {
    #[inline(always)]
    fn default() -> SlowCaCmd {
        SlowCaCmd(0)
    }
}
#[doc = "Slow cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlowCaCtl(pub u32);
impl SlowCaCtl {
    #[doc = "Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub const fn way(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_way(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub const fn set_addr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SlowCaCtl {
    #[inline(always)]
    fn default() -> SlowCaCtl {
        SlowCaCtl(0)
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smif0Ctl(pub u32);
impl Smif0Ctl {
    #[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub const fn xip_mode(&self) -> XipMode {
        let val = (self.0 >> 0usize) & 0x01;
        XipMode::from_bits(val as u8)
    }
    #[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn set_xip_mode(&mut self, val: XipMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub const fn clock_if_rx_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub fn set_clock_if_rx_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub const fn deselect_delay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn set_deselect_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub const fn block(&self) -> Block {
        let val = (self.0 >> 24usize) & 0x01;
        Block::from_bits(val as u8)
    }
    #[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn set_block(&mut self, val: Block) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub const fn enabled(&self) -> Enabled {
        let val = (self.0 >> 31usize) & 0x01;
        Enabled::from_bits(val as u8)
    }
    #[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: Enabled) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Smif0Ctl {
    #[inline(always)]
    fn default() -> Smif0Ctl {
        Smif0Ctl(0)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Transmitter command FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCmdFifoStatus(pub u32);
impl TxCmdFifoStatus {
    #[doc = "Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
    #[inline(always)]
    pub const fn used3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
    #[inline(always)]
    pub fn set_used3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for TxCmdFifoStatus {
    #[inline(always)]
    fn default() -> TxCmdFifoStatus {
        TxCmdFifoStatus(0)
    }
}
#[doc = "Transmitter command FIFO write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCmdFifoWr(pub u32);
impl TxCmdFifoWr {
    #[doc = "Command data. The higher two bits DATA\\[19:18\\] specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\] specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\] specifies which of the four devices are selected. DATA\\[11:8\\] are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\] specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\] specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
    #[inline(always)]
    pub const fn data20(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Command data. The higher two bits DATA\\[19:18\\] specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\] specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\] specifies which of the four devices are selected. DATA\\[11:8\\] are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\] specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\] specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
    #[inline(always)]
    pub fn set_data20(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for TxCmdFifoWr {
    #[inline(always)]
    fn default() -> TxCmdFifoWr {
        TxCmdFifoWr(0)
    }
}
#[doc = "Transmitter data FIFO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDataFifoCtl(pub u32);
impl TxDataFifoCtl {
    #[doc = "Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED <= TRIGGER_LEVEL."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED <= TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for TxDataFifoCtl {
    #[inline(always)]
    fn default() -> TxDataFifoCtl {
        TxDataFifoCtl(0)
    }
}
#[doc = "Transmitter data FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDataFifoStatus(pub u32);
impl TxDataFifoStatus {
    #[doc = "Number of entries that are used in the TX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub const fn used4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of entries that are used in the TX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn set_used4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TxDataFifoStatus {
    #[inline(always)]
    fn default() -> TxDataFifoStatus {
        TxDataFifoStatus(0)
    }
}
#[doc = "Transmitter data FIFO write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDataFifoWr1(pub u32);
impl TxDataFifoWr1 {
    #[doc = "TX data (written to TX data FIFO)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TxDataFifoWr1 {
    #[inline(always)]
    fn default() -> TxDataFifoWr1 {
        TxDataFifoWr1(0)
    }
}
#[doc = "Transmitter data FIFO write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDataFifoWr2(pub u32);
impl TxDataFifoWr2 {
    #[doc = "TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for TxDataFifoWr2 {
    #[inline(always)]
    fn default() -> TxDataFifoWr2 {
        TxDataFifoWr2(0)
    }
}
#[doc = "Transmitter data FIFO write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDataFifoWr4(pub u32);
impl TxDataFifoWr4 {
    #[doc = "TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "TX data (written to TX data FIFO, third byte)."]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, third byte)."]
    #[inline(always)]
    pub fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "TX data (written to TX data FIFO, fourth byte)."]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "TX data (written to TX data FIFO, fourth byte)."]
    #[inline(always)]
    pub fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxDataFifoWr4 {
    #[inline(always)]
    fn default() -> TxDataFifoWr4 {
        TxDataFifoWr4(0)
    }
}
#[doc = "Write address control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrAddrCtl(pub u32);
impl WrAddrCtl {
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for WrAddrCtl {
    #[inline(always)]
    fn default() -> WrAddrCtl {
        WrAddrCtl(0)
    }
}
#[doc = "Write command control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrCmdCtl(pub u32);
impl WrCmdCtl {
    #[doc = "Command byte code."]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command byte code."]
    #[inline(always)]
    pub fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for WrCmdCtl {
    #[inline(always)]
    fn default() -> WrCmdCtl {
        WrCmdCtl(0)
    }
}
#[doc = "Write data control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrDataCtl(pub u32);
impl WrDataCtl {
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for WrDataCtl {
    #[inline(always)]
    fn default() -> WrDataCtl {
        WrDataCtl(0)
    }
}
#[doc = "Write dummy control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrDummyCtl(pub u32);
impl WrDummyCtl {
    #[doc = "Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    pub const fn size5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    pub fn set_size5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for WrDummyCtl {
    #[inline(always)]
    fn default() -> WrDummyCtl {
        WrDummyCtl(0)
    }
}
#[doc = "Write mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrModeCtl(pub u32);
impl WrModeCtl {
    #[doc = "Mode byte code."]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Mode byte code."]
    #[inline(always)]
    pub fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Width of transfer."]
    #[inline(always)]
    pub fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub const fn present(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn set_present(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for WrModeCtl {
    #[inline(always)]
    fn default() -> WrModeCtl {
        WrModeCtl(0)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Block {
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BUS_ERROR = 0,
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    WAIT_STATES = 0x01,
}
impl Block {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Block {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Block {
    #[inline(always)]
    fn from(val: u8) -> Block {
        Block::from_bits(val)
    }
}
impl From<Block> for u8 {
    #[inline(always)]
    fn from(val: Block) -> u8 {
        Block::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enabled {
    #[doc = "N/A"]
    DISABLED = 0,
    #[doc = "N/A"]
    ENABLED = 0x01,
}
impl Enabled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enabled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enabled {
    #[inline(always)]
    fn from(val: u8) -> Enabled {
        Enabled::from_bits(val)
    }
}
impl From<Enabled> for u8 {
    #[inline(always)]
    fn from(val: Enabled) -> u8 {
        Enabled::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum XipMode {
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    MMIO_MODE = 0,
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    XIP_MODE = 0x01,
}
impl XipMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XipMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XipMode {
    #[inline(always)]
    fn from(val: u8) -> XipMode {
        XipMode::from_bits(val)
    }
}
impl From<XipMode> for u8 {
    #[inline(always)]
    fn from(val: XipMode) -> u8 {
        XipMode::to_bits(val)
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
