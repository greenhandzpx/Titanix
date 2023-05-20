#[doc = "Register `core_plloutdiv` reader"]
pub struct R(crate::R<CORE_PLLOUTDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_PLLOUTDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_PLLOUTDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_PLLOUTDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_plloutdiv` writer"]
pub struct W(crate::W<CORE_PLLOUTDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_PLLOUTDIV_SPEC>;
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
impl From<crate::W<CORE_PLLOUTDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_PLLOUTDIV_SPEC>) -> Self {
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
#[doc = "PLL Final Divide Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_plloutdiv](index.html) module"]
pub struct CORE_PLLOUTDIV_SPEC;
impl crate::RegisterSpec for CORE_PLLOUTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_plloutdiv::R](R) reader structure"]
impl crate::Readable for CORE_PLLOUTDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_plloutdiv::W](W) writer structure"]
impl crate::Writable for CORE_PLLOUTDIV_SPEC {
    type Writer = W;
}
