#[doc = "Reader of register CQCAP"]
pub type R = crate::R<u32, super::CQCAP>;
#[doc = "Reader of field `ITCFVAL`"]
pub type ITCFVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `ITCFMUL`"]
pub type ITCFMUL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CRYPTO_SUPPORT`"]
pub type CRYPTO_SUPPORT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
    #[inline(always)]
    pub fn itcfval(&self) -> ITCFVAL_R {
        ITCFVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn itcfmul(&self) -> ITCFMUL_R {
        ITCFMUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: - 0x0 (FALSE): Crypto not Supported - 0x1 (TRUE): Crypto Supported"]
    #[inline(always)]
    pub fn crypto_support(&self) -> CRYPTO_SUPPORT_R {
        CRYPTO_SUPPORT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
