#[doc = "Register `gemgxl_pllcfg` reader"]
pub struct R(crate::R<GEMGXL_PLLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEMGXL_PLLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEMGXL_PLLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEMGXL_PLLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gemgxl_pllcfg` writer"]
pub struct W(crate::W<GEMGXL_PLLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEMGXL_PLLCFG_SPEC>;
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
impl From<crate::W<GEMGXL_PLLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEMGXL_PLLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pllr` reader - PLL R Value"]
pub struct PLLR_R(crate::FieldReader<u8, u8>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllr` writer - PLL R Value"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `pllf` reader - PLL F Value"]
pub struct PLLF_R(crate::FieldReader<u16, u16>);
impl PLLF_R {
    pub(crate) fn new(bits: u16) -> Self {
        PLLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllf` writer - PLL F Value"]
pub struct PLLF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Field `pllq` reader - PLL Q Value"]
pub struct PLLQ_R(crate::FieldReader<u8, u8>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllq` writer - PLL Q Value"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `pllrange` reader - PLL Range Value"]
pub struct PLLRANGE_R(crate::FieldReader<u8, u8>);
impl PLLRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllrange` writer - PLL Range Value"]
pub struct PLLRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `pllbypass` reader - PLL Bypass"]
pub struct PLLBYPASS_R(crate::FieldReader<bool, bool>);
impl PLLBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllbypass` writer - PLL Bypass"]
pub struct PLLBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pllfsebypass` reader - PLL FSE Bypass"]
pub struct PLLFSEBYPASS_R(crate::FieldReader<bool, bool>);
impl PLLFSEBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLFSEBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLFSEBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pllfsebypass` writer - PLL FSE Bypass"]
pub struct PLLFSEBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFSEBYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `plllock` reader - PLL Lock"]
pub struct PLLLOCK_R(crate::FieldReader<bool, bool>);
impl PLLLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - PLL R Value"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLL F Value"]
    #[inline(always)]
    pub fn pllf(&self) -> PLLF_R {
        PLLF_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 15:17 - PLL Q Value"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - PLL Range Value"]
    #[inline(always)]
    pub fn pllrange(&self) -> PLLRANGE_R {
        PLLRANGE_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 24 - PLL Bypass"]
    #[inline(always)]
    pub fn pllbypass(&self) -> PLLBYPASS_R {
        PLLBYPASS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PLL FSE Bypass"]
    #[inline(always)]
    pub fn pllfsebypass(&self) -> PLLFSEBYPASS_R {
        PLLFSEBYPASS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PLL Lock"]
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL R Value"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Bits 6:14 - PLL F Value"]
    #[inline(always)]
    pub fn pllf(&mut self) -> PLLF_W {
        PLLF_W { w: self }
    }
    #[doc = "Bits 15:17 - PLL Q Value"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bits 18:20 - PLL Range Value"]
    #[inline(always)]
    pub fn pllrange(&mut self) -> PLLRANGE_W {
        PLLRANGE_W { w: self }
    }
    #[doc = "Bit 24 - PLL Bypass"]
    #[inline(always)]
    pub fn pllbypass(&mut self) -> PLLBYPASS_W {
        PLLBYPASS_W { w: self }
    }
    #[doc = "Bit 25 - PLL FSE Bypass"]
    #[inline(always)]
    pub fn pllfsebypass(&mut self) -> PLLFSEBYPASS_W {
        PLLFSEBYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gemgxl_pllcfg](index.html) module"]
pub struct GEMGXL_PLLCFG_SPEC;
impl crate::RegisterSpec for GEMGXL_PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gemgxl_pllcfg::R](R) reader structure"]
impl crate::Readable for GEMGXL_PLLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gemgxl_pllcfg::W](W) writer structure"]
impl crate::Writable for GEMGXL_PLLCFG_SPEC {
    type Writer = W;
}
