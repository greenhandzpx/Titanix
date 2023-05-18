#[doc = "Register `sampledel` reader"]
pub struct R(crate::R<SAMPLEDEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLEDEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLEDEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLEDEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sampledel` writer"]
pub struct W(crate::W<SAMPLEDEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLEDEL_SPEC>;
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
impl From<crate::W<SAMPLEDEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLEDEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd` reader - Number of delay stages from slave to SPI controller"]
pub struct SD_R(crate::FieldReader<u8, u8>);
impl SD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd` writer - Number of delay stages from slave to SPI controller"]
pub struct SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of delay stages from slave to SPI controller"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of delay stages from slave to SPI controller"]
    #[inline(always)]
    pub fn sd(&mut self) -> SD_W {
        SD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of delay stages from slave to the SPI controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampledel](index.html) module"]
pub struct SAMPLEDEL_SPEC;
impl crate::RegisterSpec for SAMPLEDEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampledel::R](R) reader structure"]
impl crate::Readable for SAMPLEDEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampledel::W](W) writer structure"]
impl crate::Writable for SAMPLEDEL_SPEC {
    type Writer = W;
}
