#[doc = "Register `delay1` reader"]
pub struct R(crate::R<DELAY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `delay1` writer"]
pub struct W(crate::W<DELAY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY1_SPEC>;
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
impl From<crate::W<DELAY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `intercs` reader - Minimum CS inactive time"]
pub struct INTERCS_R(crate::FieldReader<u8, u8>);
impl INTERCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTERCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intercs` writer - Minimum CS inactive time"]
pub struct INTERCS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `interxfr` reader - Maximum interframe delay"]
pub struct INTERXFR_R(crate::FieldReader<u8, u8>);
impl INTERXFR_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTERXFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERXFR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interxfr` writer - Maximum interframe delay"]
pub struct INTERXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERXFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&self) -> INTERCS_R {
        INTERCS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&self) -> INTERXFR_R {
        INTERXFR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum CS inactive time"]
    #[inline(always)]
    pub fn intercs(&mut self) -> INTERCS_W {
        INTERCS_W { w: self }
    }
    #[doc = "Bits 16:23 - Maximum interframe delay"]
    #[inline(always)]
    pub fn interxfr(&mut self) -> INTERXFR_W {
        INTERXFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay1](index.html) module"]
pub struct DELAY1_SPEC;
impl crate::RegisterSpec for DELAY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay1::R](R) reader structure"]
impl crate::Readable for DELAY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay1::W](W) writer structure"]
impl crate::Writable for DELAY1_SPEC {
    type Writer = W;
}
