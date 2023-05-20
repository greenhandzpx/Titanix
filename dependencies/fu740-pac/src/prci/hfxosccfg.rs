#[doc = "Register `hfxosccfg` reader"]
pub struct R(crate::R<HFXOSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfxosccfg` writer"]
pub struct W(crate::W<HFXOSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSCCFG_SPEC>;
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
impl From<crate::W<HFXOSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hfxoscen` reader - Crystal Oscillator Enable"]
pub struct HFXOSCEN_R(crate::FieldReader<bool, bool>);
impl HFXOSCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXOSCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXOSCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hfxoscen` writer - Crystal Oscillator Enable"]
pub struct HFXOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOSCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `hfxoscrdy` reader - Crystal Oscillator Ready"]
pub struct HFXOSCRDY_R(crate::FieldReader<bool, bool>);
impl HFXOSCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 30 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn hfxoscen(&self) -> HFXOSCEN_R {
        HFXOSCEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn hfxoscrdy(&self) -> HFXOSCRDY_R {
        HFXOSCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn hfxoscen(&mut self) -> HFXOSCEN_W {
        HFXOSCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal Oscillator Configuration and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosccfg](index.html) module"]
pub struct HFXOSCCFG_SPEC;
impl crate::RegisterSpec for HFXOSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosccfg::R](R) reader structure"]
impl crate::Readable for HFXOSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosccfg::W](W) writer structure"]
impl crate::Writable for HFXOSCCFG_SPEC {
    type Writer = W;
}
