#[doc = "Reader of register MS_CTL"]
pub type R = crate::R<u32, super::MS_CTL>;
#[doc = "Writer for register MS_CTL"]
pub type W = crate::W<u32, super::MS_CTL>;
#[doc = "Register MS_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MS_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PC_SAVED`"]
pub type PC_SAVED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC_SAVED`"]
pub struct PC_SAVED_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_SAVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    pub fn pc_saved(&mut self) -> PC_SAVED_W {
        PC_SAVED_W { w: self }
    }
}
