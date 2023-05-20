#[doc = "Register `claimplete_1` reader"]
pub struct R(crate::R<CLAIMPLETE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMPLETE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMPLETE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMPLETE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `claimplete_1` writer"]
pub struct W(crate::W<CLAIMPLETE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMPLETE_1_SPEC>;
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
impl From<crate::W<CLAIMPLETE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMPLETE_1_SPEC>) -> Self {
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
#[doc = "CLAIM and COMPLETE Register for hart 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimplete_1](index.html) module"]
pub struct CLAIMPLETE_1_SPEC;
impl crate::RegisterSpec for CLAIMPLETE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimplete_1::R](R) reader structure"]
impl crate::Readable for CLAIMPLETE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimplete_1::W](W) writer structure"]
impl crate::Writable for CLAIMPLETE_1_SPEC {
    type Writer = W;
}
