#[doc = "Reader of register CQSSC1"]
pub type R = crate::R<u32, super::CQSSC1>;
#[doc = "Writer for register CQSSC1"]
pub type W = crate::W<u32, super::CQSSC1>;
#[doc = "Register CQSSC1 `reset()`'s with value 0x0001_1000"]
impl crate::ResetValue for super::CQSSC1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_1000
    }
}
#[doc = "Reader of field `SQSCMD_IDLE_TMR`"]
pub type SQSCMD_IDLE_TMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SQSCMD_IDLE_TMR`"]
pub struct SQSCMD_IDLE_TMR_W<'a> {
    w: &'a mut W,
}
impl<'a> SQSCMD_IDLE_TMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SQSCMD_BLK_CNT`"]
pub type SQSCMD_BLK_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQSCMD_BLK_CNT`"]
pub struct SQSCMD_BLK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SQSCMD_BLK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
    #[inline(always)]
    pub fn sqscmd_idle_tmr(&self) -> SQSCMD_IDLE_TMR_R {
        SQSCMD_IDLE_TMR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
    #[inline(always)]
    pub fn sqscmd_blk_cnt(&self) -> SQSCMD_BLK_CNT_R {
        SQSCMD_BLK_CNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
    #[inline(always)]
    pub fn sqscmd_idle_tmr(&mut self) -> SQSCMD_IDLE_TMR_W {
        SQSCMD_IDLE_TMR_W { w: self }
    }
    #[doc = "Bits 16:19 - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
    #[inline(always)]
    pub fn sqscmd_blk_cnt(&mut self) -> SQSCMD_BLK_CNT_W {
        SQSCMD_BLK_CNT_W { w: self }
    }
}
