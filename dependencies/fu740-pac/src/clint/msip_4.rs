#[doc = "Register `msip_4` reader"]
pub struct R(crate::R<MSIP_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIP_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIP_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIP_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msip_4` writer"]
pub struct W(crate::W<MSIP_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSIP_4_SPEC>;
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
impl From<crate::W<MSIP_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSIP_4_SPEC>) -> Self {
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
#[doc = "MSIP Register for hart 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msip_4](index.html) module"]
pub struct MSIP_4_SPEC;
impl crate::RegisterSpec for MSIP_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msip_4::R](R) reader structure"]
impl crate::Readable for MSIP_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msip_4::W](W) writer structure"]
impl crate::Writable for MSIP_4_SPEC {
    type Writer = W;
}
