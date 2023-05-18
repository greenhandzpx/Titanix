#[doc = "Register `extradel` reader"]
pub struct R(crate::R<EXTRADEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTRADEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTRADEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTRADEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `extradel` writer"]
pub struct W(crate::W<EXTRADEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTRADEL_SPEC>;
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
impl From<crate::W<EXTRADEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTRADEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coarse` reader - Coarse grain sample delay (multiples of system clocks)"]
pub struct COARSE_R(crate::FieldReader<u16, u16>);
impl COARSE_R {
    pub(crate) fn new(bits: u16) -> Self {
        COARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COARSE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coarse` writer - Coarse grain sample delay (multiples of system clocks)"]
pub struct COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `fine` reader - Fine grain sample delay (multiples of process-specific buffer delay)"]
pub struct FINE_R(crate::FieldReader<u8, u8>);
impl FINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fine` writer - Fine grain sample delay (multiples of process-specific buffer delay)"]
pub struct FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Coarse grain sample delay (multiples of system clocks)"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:16 - Fine grain sample delay (multiples of process-specific buffer delay)"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Coarse grain sample delay (multiples of system clocks)"]
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W {
        COARSE_W { w: self }
    }
    #[doc = "Bits 12:16 - Fine grain sample delay (multiples of process-specific buffer delay)"]
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W {
        FINE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI extra sampling delay to increase the SPI frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extradel](index.html) module"]
pub struct EXTRADEL_SPEC;
impl crate::RegisterSpec for EXTRADEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extradel::R](R) reader structure"]
impl crate::Readable for EXTRADEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extradel::W](W) writer structure"]
impl crate::Writable for EXTRADEL_SPEC {
    type Writer = W;
}
