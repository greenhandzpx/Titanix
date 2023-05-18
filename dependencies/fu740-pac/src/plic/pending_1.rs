#[doc = "Register `pending_1` reader"]
pub struct R(crate::R<PENDING_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pending_1` writer"]
pub struct W(crate::W<PENDING_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PENDING_1_SPEC>;
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
impl From<crate::W<PENDING_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PENDING_1_SPEC>) -> Self {
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
#[doc = "PENDING Register for interrupt ids 63 to 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending_1](index.html) module"]
pub struct PENDING_1_SPEC;
impl crate::RegisterSpec for PENDING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending_1::R](R) reader structure"]
impl crate::Readable for PENDING_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pending_1::W](W) writer structure"]
impl crate::Writable for PENDING_1_SPEC {
    type Writer = W;
}
