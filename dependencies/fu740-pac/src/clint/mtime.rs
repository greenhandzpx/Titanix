#[doc = "Register `mtime` reader"]
pub struct R(crate::R<MTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mtime` writer"]
pub struct W(crate::W<MTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIME_SPEC>;
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
impl From<crate::W<MTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIME_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTIME Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime](index.html) module"]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtime::R](R) reader structure"]
impl crate::Readable for MTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtime::W](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    type Writer = W;
}
