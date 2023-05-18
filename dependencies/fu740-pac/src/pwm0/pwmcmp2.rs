#[doc = "Register `pwmcmp2` reader"]
pub struct R(crate::R<PWMCMP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCMP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCMP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCMP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwmcmp2` writer"]
pub struct W(crate::W<PWMCMP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCMP2_SPEC>;
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
impl From<crate::W<PWMCMP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCMP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwmcmp2` reader - PWM 2 Compare Value"]
pub struct PWMCMP2_R(crate::FieldReader<u16, u16>);
impl PWMCMP2_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWMCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp2` writer - PWM 2 Compare Value"]
pub struct PWMCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM 2 Compare Value"]
    #[inline(always)]
    pub fn pwmcmp2(&self) -> PWMCMP2_R {
        PWMCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM 2 Compare Value"]
    #[inline(always)]
    pub fn pwmcmp2(&mut self) -> PWMCMP2_W {
        PWMCMP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM 2 compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcmp2](index.html) module"]
pub struct PWMCMP2_SPEC;
impl crate::RegisterSpec for PWMCMP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcmp2::R](R) reader structure"]
impl crate::Readable for PWMCMP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcmp2::W](W) writer structure"]
impl crate::Writable for PWMCMP2_SPEC {
    type Writer = W;
}
