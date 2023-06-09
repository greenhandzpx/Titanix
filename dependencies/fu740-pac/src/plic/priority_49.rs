#[doc = "Register `priority_49` reader"]
pub struct R(crate::R<PRIORITY_49_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIORITY_49_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIORITY_49_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIORITY_49_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `priority_49` writer"]
pub struct W(crate::W<PRIORITY_49_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIORITY_49_SPEC>;
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
impl From<crate::W<PRIORITY_49_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIORITY_49_SPEC>) -> Self {
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
#[doc = "PRIORITY Register for interrupt id 49\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priority_49](index.html) module"]
pub struct PRIORITY_49_SPEC;
impl crate::RegisterSpec for PRIORITY_49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priority_49::R](R) reader structure"]
impl crate::Readable for PRIORITY_49_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priority_49::W](W) writer structure"]
impl crate::Writable for PRIORITY_49_SPEC {
    type Writer = W;
}
