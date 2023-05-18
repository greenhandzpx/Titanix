#[doc = "Register `ds` reader"]
pub struct R(crate::R<DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ds` writer"]
pub struct W(crate::W<DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DS_SPEC>;
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
impl From<crate::W<DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DS_SPEC>) -> Self {
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
#[doc = "Pin drive strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds](index.html) module"]
pub struct DS_SPEC;
impl crate::RegisterSpec for DS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ds::R](R) reader structure"]
impl crate::Readable for DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ds::W](W) writer structure"]
impl crate::Writable for DS_SPEC {
    type Writer = W;
}
