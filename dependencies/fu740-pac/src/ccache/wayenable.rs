#[doc = "Register `wayenable` reader"]
pub struct R(crate::R<WAYENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAYENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAYENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAYENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wayenable` writer"]
pub struct W(crate::W<WAYENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAYENABLE_SPEC>;
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
impl From<crate::W<WAYENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAYENABLE_SPEC>) -> Self {
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
#[doc = "The index of the largest way which has been enabled. May only be increased.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wayenable](index.html) module"]
pub struct WAYENABLE_SPEC;
impl crate::RegisterSpec for WAYENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wayenable::R](R) reader structure"]
impl crate::Readable for WAYENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wayenable::W](W) writer structure"]
impl crate::Writable for WAYENABLE_SPEC {
    type Writer = W;
}
