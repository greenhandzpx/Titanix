#[doc = "Register `core_clk_sel_reg` reader"]
pub struct R(crate::R<CORE_CLK_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CLK_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CLK_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CLK_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core_clk_sel_reg` writer"]
pub struct W(crate::W<CORE_CLK_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CLK_SEL_REG_SPEC>;
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
impl From<crate::W<CORE_CLK_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CLK_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Core clock source"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: Select core_pll mux output"]
    PLL_MUX = 0,
    #[doc = "1: Select hfclk clock"]
    HFCLK = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `source` reader - Core clock source"]
pub struct SOURCE_R(crate::FieldReader<bool, SOURCE_A>);
impl SOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::PLL_MUX,
            true => SOURCE_A::HFCLK,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_MUX`"]
    #[inline(always)]
    pub fn is_pll_mux(&self) -> bool {
        **self == SOURCE_A::PLL_MUX
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        **self == SOURCE_A::HFCLK
    }
}
impl core::ops::Deref for SOURCE_R {
    type Target = crate::FieldReader<bool, SOURCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `source` writer - Core clock source"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select core_pll mux output"]
    #[inline(always)]
    pub fn pll_mux(self) -> &'a mut W {
        self.variant(SOURCE_A::PLL_MUX)
    }
    #[doc = "Select hfclk clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(SOURCE_A::HFCLK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Core clock source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core clock source"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core clock source register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_clk_sel_reg](index.html) module"]
pub struct CORE_CLK_SEL_REG_SPEC;
impl crate::RegisterSpec for CORE_CLK_SEL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_clk_sel_reg::R](R) reader structure"]
impl crate::Readable for CORE_CLK_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_clk_sel_reg::W](W) writer structure"]
impl crate::Writable for CORE_CLK_SEL_REG_SPEC {
    type Writer = W;
}
