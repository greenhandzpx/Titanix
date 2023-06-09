#[doc = "Register `csid` reader"]
pub struct R(crate::R<CSID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csid` writer"]
pub struct W(crate::W<CSID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSID_SPEC>;
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
impl From<crate::W<CSID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csid` reader - Chip select ID."]
pub struct CSID_R(crate::FieldReader<u32, u32>);
impl CSID_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `csid` writer - Chip select ID."]
pub struct CSID_W<'a> {
    w: &'a mut W,
}
impl<'a> CSID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip select ID."]
    #[inline(always)]
    pub fn csid(&self) -> CSID_R {
        CSID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Chip select ID."]
    #[inline(always)]
    pub fn csid(&mut self) -> CSID_W {
        CSID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip select ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csid](index.html) module"]
pub struct CSID_SPEC;
impl crate::RegisterSpec for CSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csid::R](R) reader structure"]
impl crate::Readable for CSID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csid::W](W) writer structure"]
impl crate::Writable for CSID_SPEC {
    type Writer = W;
}
