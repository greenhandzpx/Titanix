#[doc = "Register `high_ip` reader"]
pub struct R(crate::R<HIGH_IP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGH_IP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGH_IP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGH_IP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `high_ip` writer"]
pub struct W(crate::W<HIGH_IP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIGH_IP_SPEC>;
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
impl From<crate::W<HIGH_IP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIGH_IP_SPEC>) -> Self {
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
#[doc = "High interrupt pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [high_ip](index.html) module"]
pub struct HIGH_IP_SPEC;
impl crate::RegisterSpec for HIGH_IP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [high_ip::R](R) reader structure"]
impl crate::Readable for HIGH_IP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [high_ip::W](W) writer structure"]
impl crate::Writable for HIGH_IP_SPEC {
    type Writer = W;
}
