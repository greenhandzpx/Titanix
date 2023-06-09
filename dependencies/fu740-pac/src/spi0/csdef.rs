#[doc = "Register `csdef` reader"]
pub struct R(crate::R<CSDEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSDEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSDEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSDEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csdef` writer"]
pub struct W(crate::W<CSDEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSDEF_SPEC>;
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
impl From<crate::W<CSDEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSDEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csdef` reader - Chip select default value. Reset to all-1s."]
pub struct CSDEF_R(crate::FieldReader<u32, u32>);
impl CSDEF_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDEF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `csdef` writer - Chip select default value. Reset to all-1s."]
pub struct CSDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip select default value. Reset to all-1s."]
    #[inline(always)]
    pub fn csdef(&self) -> CSDEF_R {
        CSDEF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Chip select default value. Reset to all-1s."]
    #[inline(always)]
    pub fn csdef(&mut self) -> CSDEF_W {
        CSDEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip select default\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdef](index.html) module"]
pub struct CSDEF_SPEC;
impl crate::RegisterSpec for CSDEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csdef::R](R) reader structure"]
impl crate::Readable for CSDEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csdef::W](W) writer structure"]
impl crate::Writable for CSDEF_SPEC {
    type Writer = W;
}
