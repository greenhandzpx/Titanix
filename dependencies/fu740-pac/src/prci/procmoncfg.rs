#[doc = "Register `procmoncfg` reader"]
pub struct R(crate::R<PROCMONCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCMONCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCMONCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCMONCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `procmoncfg` writer"]
pub struct W(crate::W<PROCMONCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROCMONCFG_SPEC>;
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
impl From<crate::W<PROCMONCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROCMONCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core_clock` reader - "]
pub struct CORE_CLOCK_R(crate::FieldReader<bool, bool>);
impl CORE_CLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_CLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_CLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `core_clock` writer - "]
pub struct CORE_CLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_CLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn core_clock(&self) -> CORE_CLOCK_R {
        CORE_CLOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn core_clock(&mut self) -> CORE_CLOCK_W {
        CORE_CLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procmoncfg](index.html) module"]
pub struct PROCMONCFG_SPEC;
impl crate::RegisterSpec for PROCMONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [procmoncfg::R](R) reader structure"]
impl crate::Readable for PROCMONCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [procmoncfg::W](W) writer structure"]
impl crate::Writable for PROCMONCFG_SPEC {
    type Writer = W;
}
