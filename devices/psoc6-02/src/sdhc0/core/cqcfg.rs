#[doc = "Reader of register CQCFG"]
pub type R = crate::R<u32, super::CQCFG>;
#[doc = "Writer for register CQCFG"]
pub type W = crate::W<u32, super::CQCFG>;
#[doc = "Register CQCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCFG {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `CQ_EN`"]
pub type CQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CQ_EN`"]
pub struct CQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQ_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CR_GENERAL_EN`"]
pub type CR_GENERAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR_GENERAL_EN`"]
pub struct CR_GENERAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_GENERAL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TASK_DESC_SIZE`"]
pub type TASK_DESC_SIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TASK_DESC_SIZE`"]
pub struct TASK_DESC_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TASK_DESC_SIZE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DCMD_EN`"]
pub type DCMD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMD_EN`"]
pub struct DCMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    pub fn cq_en(&self) -> CQ_EN_R {
        CQ_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn cr_general_en(&self) -> CR_GENERAL_EN_R {
        CR_GENERAL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TASK_DESC_SIZE_R {
        TASK_DESC_SIZE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&self) -> DCMD_EN_R {
        DCMD_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    pub fn cq_en(&mut self) -> CQ_EN_W {
        CQ_EN_W { w: self }
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn cr_general_en(&mut self) -> CR_GENERAL_EN_W {
        CR_GENERAL_EN_W { w: self }
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&mut self) -> TASK_DESC_SIZE_W {
        TASK_DESC_SIZE_W { w: self }
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&mut self) -> DCMD_EN_W {
        DCMD_EN_W { w: self }
    }
}
