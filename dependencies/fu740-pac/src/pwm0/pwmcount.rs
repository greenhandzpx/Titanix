#[doc = "Register `pwmcount` reader"]
pub struct R(crate::R<PWMCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwmcount` writer"]
pub struct W(crate::W<PWMCOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCOUNT_SPEC>;
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
impl From<crate::W<PWMCOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwmcount` reader - PWM count register."]
pub struct PWMCOUNT_R(crate::FieldReader<u32, u32>);
impl PWMCOUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        PWMCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcount` writer - PWM count register."]
pub struct PWMCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - PWM count register."]
    #[inline(always)]
    pub fn pwmcount(&self) -> PWMCOUNT_R {
        PWMCOUNT_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - PWM count register."]
    #[inline(always)]
    pub fn pwmcount(&mut self) -> PWMCOUNT_W {
        PWMCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcount](index.html) module"]
pub struct PWMCOUNT_SPEC;
impl crate::RegisterSpec for PWMCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcount::R](R) reader structure"]
impl crate::Readable for PWMCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcount::W](W) writer structure"]
impl crate::Writable for PWMCOUNT_SPEC {
    type Writer = W;
}
