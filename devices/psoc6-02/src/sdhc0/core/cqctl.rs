#[doc = "Reader of register CQCTL"]
pub type R = crate::R<u32, super::CQCTL>;
#[doc = "Writer for register CQCTL"]
pub type W = crate::W<u32, super::CQCTL>;
#[doc = "Register CQCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCTL {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
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
#[doc = "Reader of field `CLR_ALL_TASKS`"]
pub type CLR_ALL_TASKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_ALL_TASKS`"]
pub struct CLR_ALL_TASKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_ALL_TASKS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    pub fn clr_all_tasks(&self) -> CLR_ALL_TASKS_R {
        CLR_ALL_TASKS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    pub fn clr_all_tasks(&mut self) -> CLR_ALL_TASKS_W {
        CLR_ALL_TASKS_W { w: self }
    }
}
