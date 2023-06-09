#[doc = "Register `fmt` reader"]
pub struct R(crate::R<FMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fmt` writer"]
pub struct W(crate::W<FMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMT_SPEC>;
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
impl From<crate::W<FMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `proto` reader - SPI protocol"]
pub struct PROTO_R(crate::FieldReader<u8, u8>);
impl PROTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `proto` writer - SPI protocol"]
pub struct PROTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `endian` reader - SPI endianness"]
pub struct ENDIAN_R(crate::FieldReader<bool, bool>);
impl ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `endian` writer - SPI endianness"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
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
#[doc = "Field `dir` reader - SPI I/O direction. This is reset to 1 for flash-enabled SPI controllers, 0 otherwise."]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dir` writer - SPI I/O direction. This is reset to 1 for flash-enabled SPI controllers, 0 otherwise."]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Field `len` reader - Number of bits per frame"]
pub struct LEN_R(crate::FieldReader<u8, u8>);
impl LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `len` writer - Number of bits per frame"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&self) -> PROTO_R {
        PROTO_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SPI I/O direction. This is reset to 1 for flash-enabled SPI controllers, 0 otherwise."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI protocol"]
    #[inline(always)]
    pub fn proto(&mut self) -> PROTO_W {
        PROTO_W { w: self }
    }
    #[doc = "Bit 2 - SPI endianness"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bit 3 - SPI I/O direction. This is reset to 1 for flash-enabled SPI controllers, 0 otherwise."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 16:19 - Number of bits per frame"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmt](index.html) module"]
pub struct FMT_SPEC;
impl crate::RegisterSpec for FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmt::R](R) reader structure"]
impl crate::Readable for FMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmt::W](W) writer structure"]
impl crate::Writable for FMT_SPEC {
    type Writer = W;
}
