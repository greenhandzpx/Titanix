#[doc = "Register `delay0` reader"]
pub struct R(crate::R<DELAY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `delay0` writer"]
pub struct W(crate::W<DELAY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY0_SPEC>;
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
impl From<crate::W<DELAY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cssck` reader - CS to SCK Delay"]
pub struct CSSCK_R(crate::FieldReader<u8, u8>);
impl CSSCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSSCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cssck` writer - CS to SCK Delay"]
pub struct CSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `sckcs` reader - SCK to CS Delay"]
pub struct SCKCS_R(crate::FieldReader<u8, u8>);
impl SCKCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCKCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sckcs` writer - SCK to CS Delay"]
pub struct SCKCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&self) -> CSSCK_R {
        CSSCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&self) -> SCKCS_R {
        SCKCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CS to SCK Delay"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CSSCK_W {
        CSSCK_W { w: self }
    }
    #[doc = "Bits 16:23 - SCK to CS Delay"]
    #[inline(always)]
    pub fn sckcs(&mut self) -> SCKCS_W {
        SCKCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay0](index.html) module"]
pub struct DELAY0_SPEC;
impl crate::RegisterSpec for DELAY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay0::R](R) reader structure"]
impl crate::Readable for DELAY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay0::W](W) writer structure"]
impl crate::Writable for DELAY0_SPEC {
    type Writer = W;
}
