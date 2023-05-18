#[doc = "Register `iof_sel` reader"]
pub struct R(crate::R<IOF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iof_sel` writer"]
pub struct W(crate::W<IOF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOF_SEL_SPEC>;
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
impl From<crate::W<IOF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOF_SEL_SPEC>) -> Self {
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
#[doc = "I/O function select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iof_sel](index.html) module"]
pub struct IOF_SEL_SPEC;
impl crate::RegisterSpec for IOF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iof_sel::R](R) reader structure"]
impl crate::Readable for IOF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iof_sel::W](W) writer structure"]
impl crate::Writable for IOF_SEL_SPEC {
    type Writer = W;
}
