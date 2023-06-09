#[doc = "Register `MSEL` reader"]
pub struct R(crate::R<MSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MSEL pin state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msel](index.html) module"]
pub struct MSEL_SPEC;
impl crate::RegisterSpec for MSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msel::R](R) reader structure"]
impl crate::Readable for MSEL_SPEC {
    type Reader = R;
}
