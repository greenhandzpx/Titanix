#[doc = "Register `prescale_high` reader"]
pub struct R(crate::R<PRESCALE_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALE_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALE_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALE_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prescale_high` writer"]
pub struct W(crate::W<PRESCALE_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALE_HIGH_SPEC>;
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
impl From<crate::W<PRESCALE_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALE_HIGH_SPEC>) -> Self {
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
#[doc = "Clock Prescale register hi-byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescale_high](index.html) module"]
pub struct PRESCALE_HIGH_SPEC;
impl crate::RegisterSpec for PRESCALE_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescale_high::R](R) reader structure"]
impl crate::Readable for PRESCALE_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescale_high::W](W) writer structure"]
impl crate::Writable for PRESCALE_HIGH_SPEC {
    type Writer = W;
}
