#[doc = "Register `threshold_1M` reader"]
pub struct R(crate::R<THRESHOLD_1M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESHOLD_1M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESHOLD_1M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESHOLD_1M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `threshold_1M` writer"]
pub struct W(crate::W<THRESHOLD_1M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESHOLD_1M_SPEC>;
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
impl From<crate::W<THRESHOLD_1M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESHOLD_1M_SPEC>) -> Self {
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
#[doc = "PRIORITY THRESHOLD Register for hart 1 M-Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold_1M](index.html) module"]
pub struct THRESHOLD_1M_SPEC;
impl crate::RegisterSpec for THRESHOLD_1M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [threshold_1M::R](R) reader structure"]
impl crate::Readable for THRESHOLD_1M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [threshold_1M::W](W) writer structure"]
impl crate::Writable for THRESHOLD_1M_SPEC {
    type Writer = W;
}
