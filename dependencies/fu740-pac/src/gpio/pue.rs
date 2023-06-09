#[doc = "Register `pue` reader"]
pub struct R(crate::R<PUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pue` writer"]
pub struct W(crate::W<PUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUE_SPEC>;
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
impl From<crate::W<PUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUE_SPEC>) -> Self {
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
#[doc = "Internal pull-up enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pue](index.html) module"]
pub struct PUE_SPEC;
impl crate::RegisterSpec for PUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pue::R](R) reader structure"]
impl crate::Readable for PUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pue::W](W) writer structure"]
impl crate::Writable for PUE_SPEC {
    type Writer = W;
}
