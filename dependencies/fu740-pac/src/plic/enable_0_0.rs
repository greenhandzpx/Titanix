#[doc = "Register `enable_0_0` reader"]
pub struct R(crate::R<ENABLE_0_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_0_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_0_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `enable_0_0` writer"]
pub struct W(crate::W<ENABLE_0_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ENABLE_0_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_0_0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_0_0](index.html) module"]
pub struct ENABLE_0_0_SPEC;
impl crate::RegisterSpec for ENABLE_0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_0_0::R](R) reader structure"]
impl crate::Readable for ENABLE_0_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_0_0::W](W) writer structure"]
impl crate::Writable for ENABLE_0_0_SPEC {
    type Writer = W;
}
