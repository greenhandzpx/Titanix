#[doc = "Register `hfpclkpllsel` reader"]
pub struct R(crate::R<HFPCLKPLLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPCLKPLLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPCLKPLLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPCLKPLLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfpclkpllsel` writer"]
pub struct W(crate::W<HFPCLKPLLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPCLKPLLSEL_SPEC>;
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
impl From<crate::W<HFPCLKPLLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPCLKPLLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "hfpclk clock source"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: Select hfpclkpll output"]
    HFPCLKPLL = 0,
    #[doc = "1: Select hfclk clock"]
    HFCLK = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `source` reader - hfpclk clock source"]
pub struct SOURCE_R(crate::FieldReader<bool, SOURCE_A>);
impl SOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::HFPCLKPLL,
            true => SOURCE_A::HFCLK,
        }
    }
    #[doc = "Checks if the value of the field is `HFPCLKPLL`"]
    #[inline(always)]
    pub fn is_hfpclkpll(&self) -> bool {
        **self == SOURCE_A::HFPCLKPLL
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
#[doc = "Field `source` writer - hfpclk clock source"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select hfpclkpll output"]
    #[inline(always)]
    pub fn hfpclkpll(self) -> &'a mut W {
        self.variant(SOURCE_A::HFPCLKPLL)
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
    #[doc = "Bit 0 - hfpclk clock source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hfpclk clock source"]
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
#[doc = "Periphery clock source register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpclkpllsel](index.html) module"]
pub struct HFPCLKPLLSEL_SPEC;
impl crate::RegisterSpec for HFPCLKPLLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfpclkpllsel::R](R) reader structure"]
impl crate::Readable for HFPCLKPLLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfpclkpllsel::W](W) writer structure"]
impl crate::Writable for HFPCLKPLLSEL_SPEC {
    type Writer = W;
}
