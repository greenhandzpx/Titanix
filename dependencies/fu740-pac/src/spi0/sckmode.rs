#[doc = "Register `sckmode` reader"]
pub struct R(crate::R<SCKMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCKMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCKMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCKMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sckmode` writer"]
pub struct W(crate::W<SCKMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCKMODE_SPEC>;
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
impl From<crate::W<SCKMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCKMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pha` reader - Serial clock phase"]
pub struct PHA_R(crate::FieldReader<bool, bool>);
impl PHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pha` writer - Serial clock phase"]
pub struct PHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHA_W<'a> {
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
#[doc = "Field `pol` reader - Serial clock polarity"]
pub struct POL_R(crate::FieldReader<bool, bool>);
impl POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pol` writer - Serial clock polarity"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W {
        PHA_W { w: self }
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial clock mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sckmode](index.html) module"]
pub struct SCKMODE_SPEC;
impl crate::RegisterSpec for SCKMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sckmode::R](R) reader structure"]
impl crate::Readable for SCKMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sckmode::W](W) writer structure"]
impl crate::Writable for SCKMODE_SPEC {
    type Writer = W;
}
