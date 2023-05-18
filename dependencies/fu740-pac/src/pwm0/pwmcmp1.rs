#[doc = "Register `pwmcmp1` reader"]
pub struct R(crate::R<PWMCMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwmcmp1` writer"]
pub struct W(crate::W<PWMCMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCMP1_SPEC>;
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
impl From<crate::W<PWMCMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwmcmp1` reader - PWM 1 Compare Value"]
pub struct PWMCMP1_R(crate::FieldReader<u16, u16>);
impl PWMCMP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWMCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp1` writer - PWM 1 Compare Value"]
pub struct PWMCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PWM 1 Compare Value"]
    #[inline(always)]
    pub fn pwmcmp1(&self) -> PWMCMP1_R {
        PWMCMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM 1 Compare Value"]
    #[inline(always)]
    pub fn pwmcmp1(&mut self) -> PWMCMP1_W {
        PWMCMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM 1 compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcmp1](index.html) module"]
pub struct PWMCMP1_SPEC;
impl crate::RegisterSpec for PWMCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcmp1::R](R) reader structure"]
impl crate::Readable for PWMCMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcmp1::W](W) writer structure"]
impl crate::Writable for PWMCMP1_SPEC {
    type Writer = W;
}
