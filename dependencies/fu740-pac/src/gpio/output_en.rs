#[doc = "Register `output_en` reader"]
pub struct R(crate::R<OUTPUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `output_en` writer"]
pub struct W(crate::W<OUTPUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_EN_SPEC>;
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
impl From<crate::W<OUTPUT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUT_EN_SPEC>) -> Self {
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
#[doc = "Pin output enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output_en](index.html) module"]
pub struct OUTPUT_EN_SPEC;
impl crate::RegisterSpec for OUTPUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output_en::R](R) reader structure"]
impl crate::Readable for OUTPUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [output_en::W](W) writer structure"]
impl crate::Writable for OUTPUT_EN_SPEC {
    type Writer = W;
}
