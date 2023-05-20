#[doc = "Register `corepllsel` reader"]
pub struct R(crate::R<COREPLLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COREPLLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COREPLLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COREPLLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `corepllsel` writer"]
pub struct W(crate::W<COREPLLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COREPLLSEL_SPEC>;
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
impl From<crate::W<COREPLLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COREPLLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "core_pll mux clock select"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: Select corepll output"]
    COREPLL = 0,
    #[doc = "1: Select dvfscorepll output"]
    DVFSCOREPLL = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `source` reader - core_pll mux clock select"]
pub struct SOURCE_R(crate::FieldReader<bool, SOURCE_A>);
impl SOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::COREPLL,
            true => SOURCE_A::DVFSCOREPLL,
        }
    }
    #[doc = "Checks if the value of the field is `COREPLL`"]
    #[inline(always)]
    pub fn is_corepll(&self) -> bool {
        **self == SOURCE_A::COREPLL
    }
    #[doc = "Checks if the value of the field is `DVFSCOREPLL`"]
    #[inline(always)]
    pub fn is_dvfscorepll(&self) -> bool {
        **self == SOURCE_A::DVFSCOREPLL
    }
}
impl core::ops::Deref for SOURCE_R {
    type Target = crate::FieldReader<bool, SOURCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `source` writer - core_pll mux clock select"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select corepll output"]
    #[inline(always)]
    pub fn corepll(self) -> &'a mut W {
        self.variant(SOURCE_A::COREPLL)
    }
    #[doc = "Select dvfscorepll output"]
    #[inline(always)]
    pub fn dvfscorepll(self) -> &'a mut W {
        self.variant(SOURCE_A::DVFSCOREPLL)
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
    #[doc = "Bit 0 - core_pll mux clock select"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core_pll mux clock select"]
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
#[doc = "Clock select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [corepllsel](index.html) module"]
pub struct COREPLLSEL_SPEC;
impl crate::RegisterSpec for COREPLLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [corepllsel::R](R) reader structure"]
impl crate::Readable for COREPLLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [corepllsel::W](W) writer structure"]
impl crate::Writable for COREPLLSEL_SPEC {
    type Writer = W;
}
