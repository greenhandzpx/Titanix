#[doc = "Register `hfpclk_div_reg` reader"]
pub struct R(crate::R<HFPCLK_DIV_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPCLK_DIV_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPCLK_DIV_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPCLK_DIV_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfpclk_div_reg` writer"]
pub struct W(crate::W<HFPCLK_DIV_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPCLK_DIV_REG_SPEC>;
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
impl From<crate::W<HFPCLK_DIV_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPCLK_DIV_REG_SPEC>) -> Self {
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
#[doc = "HFPCLK PLL divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpclk_div_reg](index.html) module"]
pub struct HFPCLK_DIV_REG_SPEC;
impl crate::RegisterSpec for HFPCLK_DIV_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfpclk_div_reg::R](R) reader structure"]
impl crate::Readable for HFPCLK_DIV_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfpclk_div_reg::W](W) writer structure"]
impl crate::Writable for HFPCLK_DIV_REG_SPEC {
    type Writer = W;
}
