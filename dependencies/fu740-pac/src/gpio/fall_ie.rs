#[doc = "Register `fall_ie` reader"]
pub struct R(crate::R<FALL_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FALL_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FALL_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FALL_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fall_ie` writer"]
pub struct W(crate::W<FALL_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FALL_IE_SPEC>;
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
impl From<crate::W<FALL_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FALL_IE_SPEC>) -> Self {
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
#[doc = "Fall interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fall_ie](index.html) module"]
pub struct FALL_IE_SPEC;
impl crate::RegisterSpec for FALL_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fall_ie::R](R) reader structure"]
impl crate::Readable for FALL_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fall_ie::W](W) writer structure"]
impl crate::Writable for FALL_IE_SPEC {
    type Writer = W;
}
