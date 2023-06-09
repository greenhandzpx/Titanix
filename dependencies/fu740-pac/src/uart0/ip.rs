#[doc = "Register `ip` reader"]
pub struct R(crate::R<IP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `txwm` reader - Transmit watermark interrupt pending"]
pub struct TXWM_R(crate::FieldReader<bool, bool>);
impl TXWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxwm` reader - Receive watermark interrupt pending"]
pub struct RXWM_R(crate::FieldReader<bool, bool>);
impl RXWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit watermark interrupt pending"]
    #[inline(always)]
    pub fn txwm(&self) -> TXWM_R {
        TXWM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive watermark interrupt pending"]
    #[inline(always)]
    pub fn rxwm(&self) -> RXWM_R {
        RXWM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "UART interrupt pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip](index.html) module"]
pub struct IP_SPEC;
impl crate::RegisterSpec for IP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ip::R](R) reader structure"]
impl crate::Readable for IP_SPEC {
    type Reader = R;
}
