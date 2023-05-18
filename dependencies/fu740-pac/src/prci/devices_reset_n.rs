#[doc = "Register `devices_reset_n` reader"]
pub struct R(crate::R<DEVICES_RESET_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICES_RESET_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICES_RESET_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICES_RESET_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `devices_reset_n` writer"]
pub struct W(crate::W<DEVICES_RESET_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICES_RESET_N_SPEC>;
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
impl From<crate::W<DEVICES_RESET_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICES_RESET_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ddrctrl_reset_n` reader - Active-Low ddrctrl reset"]
pub struct DDRCTRL_RESET_N_R(crate::FieldReader<bool, bool>);
impl DDRCTRL_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCTRL_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCTRL_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrctrl_reset_n` writer - Active-Low ddrctrl reset"]
pub struct DDRCTRL_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCTRL_RESET_N_W<'a> {
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
#[doc = "Field `ddraxi_reset_n` reader - Active-Low ddraxi reset"]
pub struct DDRAXI_RESET_N_R(crate::FieldReader<bool, bool>);
impl DDRAXI_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRAXI_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRAXI_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddraxi_reset_n` writer - Active-Low ddraxi reset"]
pub struct DDRAXI_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRAXI_RESET_N_W<'a> {
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
#[doc = "Field `ddrahb_reset_n` reader - Active-Low ddrahb reset"]
pub struct DDRAHB_RESET_N_R(crate::FieldReader<bool, bool>);
impl DDRAHB_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRAHB_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRAHB_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrahb_reset_n` writer - Active-Low ddrahb reset"]
pub struct DDRAHB_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRAHB_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ddrphy_reset_n` reader - Active-Low ddrphy reset"]
pub struct DDRPHY_RESET_N_R(crate::FieldReader<bool, bool>);
impl DDRPHY_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHY_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHY_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrphy_reset_n` writer - Active-Low ddrphy reset"]
pub struct DDRPHY_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHY_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `pcieaux_reset_n` reader - Active-Low pcieaux reset"]
pub struct PCIEAUX_RESET_N_R(crate::FieldReader<bool, bool>);
impl PCIEAUX_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEAUX_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEAUX_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pcieaux_reset_n` writer - Active-Low pcieaux reset"]
pub struct PCIEAUX_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEAUX_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `gemgxl_reset_n` reader - Active-Low gemgxl reset"]
pub struct GEMGXL_RESET_N_R(crate::FieldReader<bool, bool>);
impl GEMGXL_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEMGXL_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEMGXL_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gemgxl_reset_n` writer - Active-Low gemgxl reset"]
pub struct GEMGXL_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GEMGXL_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Active-Low ddrctrl reset"]
    #[inline(always)]
    pub fn ddrctrl_reset_n(&self) -> DDRCTRL_RESET_N_R {
        DDRCTRL_RESET_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active-Low ddraxi reset"]
    #[inline(always)]
    pub fn ddraxi_reset_n(&self) -> DDRAXI_RESET_N_R {
        DDRAXI_RESET_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active-Low ddrahb reset"]
    #[inline(always)]
    pub fn ddrahb_reset_n(&self) -> DDRAHB_RESET_N_R {
        DDRAHB_RESET_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active-Low ddrphy reset"]
    #[inline(always)]
    pub fn ddrphy_reset_n(&self) -> DDRPHY_RESET_N_R {
        DDRPHY_RESET_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active-Low pcieaux reset"]
    #[inline(always)]
    pub fn pcieaux_reset_n(&self) -> PCIEAUX_RESET_N_R {
        PCIEAUX_RESET_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Active-Low gemgxl reset"]
    #[inline(always)]
    pub fn gemgxl_reset_n(&self) -> GEMGXL_RESET_N_R {
        GEMGXL_RESET_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active-Low ddrctrl reset"]
    #[inline(always)]
    pub fn ddrctrl_reset_n(&mut self) -> DDRCTRL_RESET_N_W {
        DDRCTRL_RESET_N_W { w: self }
    }
    #[doc = "Bit 1 - Active-Low ddraxi reset"]
    #[inline(always)]
    pub fn ddraxi_reset_n(&mut self) -> DDRAXI_RESET_N_W {
        DDRAXI_RESET_N_W { w: self }
    }
    #[doc = "Bit 2 - Active-Low ddrahb reset"]
    #[inline(always)]
    pub fn ddrahb_reset_n(&mut self) -> DDRAHB_RESET_N_W {
        DDRAHB_RESET_N_W { w: self }
    }
    #[doc = "Bit 3 - Active-Low ddrphy reset"]
    #[inline(always)]
    pub fn ddrphy_reset_n(&mut self) -> DDRPHY_RESET_N_W {
        DDRPHY_RESET_N_W { w: self }
    }
    #[doc = "Bit 4 - Active-Low pcieaux reset"]
    #[inline(always)]
    pub fn pcieaux_reset_n(&mut self) -> PCIEAUX_RESET_N_W {
        PCIEAUX_RESET_N_W { w: self }
    }
    #[doc = "Bit 5 - Active-Low gemgxl reset"]
    #[inline(always)]
    pub fn gemgxl_reset_n(&mut self) -> GEMGXL_RESET_N_W {
        GEMGXL_RESET_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software controlled resets\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devices_reset_n](index.html) module"]
pub struct DEVICES_RESET_N_SPEC;
impl crate::RegisterSpec for DEVICES_RESET_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devices_reset_n::R](R) reader structure"]
impl crate::Readable for DEVICES_RESET_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devices_reset_n::W](W) writer structure"]
impl crate::Writable for DEVICES_RESET_N_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets devices_reset_n to value 0"]
impl crate::Resettable for DEVICES_RESET_N_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
