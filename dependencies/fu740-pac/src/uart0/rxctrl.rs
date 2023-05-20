#[doc = "Register `rxctrl` reader"]
pub struct R(crate::R<RXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxctrl` writer"]
pub struct W(crate::W<RXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCTRL_SPEC>;
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
impl From<crate::W<RXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxen` reader - Receive enable"]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxen` writer - Receive enable"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
#[doc = "Field `rxcnt` reader - Receive watermark level"]
pub struct RXCNT_R(crate::FieldReader<u8, u8>);
impl RXCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxcnt` writer - Receive watermark level"]
pub struct RXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    pub fn rxcnt(&mut self) -> RXCNT_W {
        RXCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxctrl](index.html) module"]
pub struct RXCTRL_SPEC;
impl crate::RegisterSpec for RXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxctrl::R](R) reader structure"]
impl crate::Readable for RXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxctrl::W](W) writer structure"]
impl crate::Writable for RXCTRL_SPEC {
    type Writer = W;
}
