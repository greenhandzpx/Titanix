#[doc = "Register `rxmark` reader"]
pub struct R(crate::R<RXMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxmark` writer"]
pub struct W(crate::W<RXMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMARK_SPEC>;
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
impl From<crate::W<RXMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxmark` reader - Receive watermark"]
pub struct RXMARK_R(crate::FieldReader<u8, u8>);
impl RXMARK_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMARK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxmark` writer - Receive watermark"]
pub struct RXMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&self) -> RXMARK_R {
        RXMARK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive watermark"]
    #[inline(always)]
    pub fn rxmark(&mut self) -> RXMARK_W {
        RXMARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO watermark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmark](index.html) module"]
pub struct RXMARK_SPEC;
impl crate::RegisterSpec for RXMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmark::R](R) reader structure"]
impl crate::Readable for RXMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmark::W](W) writer structure"]
impl crate::Writable for RXMARK_SPEC {
    type Writer = W;
}
