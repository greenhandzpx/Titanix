#[doc = "Register `finisher` reader"]
pub struct R(crate::R<FINISHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FINISHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FINISHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FINISHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `finisher` writer"]
pub struct W(crate::W<FINISHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FINISHER_SPEC>;
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
impl From<crate::W<FINISHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FINISHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `status` reader - Test status"]
pub struct STATUS_R(crate::FieldReader<u16, u16>);
impl STATUS_R {
    pub(crate) fn new(bits: u16) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `status` writer - Test status"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `code` reader - Finisher code"]
pub struct CODE_R(crate::FieldReader<u16, u16>);
impl CODE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `code` writer - Finisher code"]
pub struct CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Test status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Finisher code"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Test status"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Bits 16:31 - Finisher code"]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test finisher register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [finisher](index.html) module"]
pub struct FINISHER_SPEC;
impl crate::RegisterSpec for FINISHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [finisher::R](R) reader structure"]
impl crate::Readable for FINISHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [finisher::W](W) writer structure"]
impl crate::Writable for FINISHER_SPEC {
    type Writer = W;
}
