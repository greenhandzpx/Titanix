#[doc = "Register `clk_mux_status` reader"]
pub struct R(crate::R<CLK_MUX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_MUX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_MUX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_MUX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `coreclkpllsel` reader - Current setting of coreclkpllsel mux"]
pub struct CORECLKPLLSEL_R(crate::FieldReader<bool, bool>);
impl CORECLKPLLSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORECLKPLLSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORECLKPLLSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tlclksel` reader - Current setting of tlclksel mux"]
pub struct TLCLKSEL_R(crate::FieldReader<bool, bool>);
impl TLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtcxsel` reader - Current setting of rtcxsel mux"]
pub struct RTCXSEL_R(crate::FieldReader<bool, bool>);
impl RTCXSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCXSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCXSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrctrlclksel` reader - Current setting of ddrctrlclksel mux"]
pub struct DDRCTRLCLKSEL_R(crate::FieldReader<bool, bool>);
impl DDRCTRLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCTRLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCTRLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrphyclksel` reader - Current setting of ddrphyclksel mux"]
pub struct DDRPHYCLKSEL_R(crate::FieldReader<bool, bool>);
impl DDRPHYCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reserved0` reader - Current setting of reserved0 mux"]
pub struct RESERVED0_R(crate::FieldReader<bool, bool>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gemgxlclksel` reader - Current setting of gemgxlclksel mux"]
pub struct GEMGXLCLKSEL_R(crate::FieldReader<bool, bool>);
impl GEMGXLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEMGXLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEMGXLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mainmemclksel` reader - Current setting of mainmemclksel mux"]
pub struct MAINMEMCLKSEL_R(crate::FieldReader<bool, bool>);
impl MAINMEMCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAINMEMCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAINMEMCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Current setting of coreclkpllsel mux"]
    #[inline(always)]
    pub fn coreclkpllsel(&self) -> CORECLKPLLSEL_R {
        CORECLKPLLSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Current setting of tlclksel mux"]
    #[inline(always)]
    pub fn tlclksel(&self) -> TLCLKSEL_R {
        TLCLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Current setting of rtcxsel mux"]
    #[inline(always)]
    pub fn rtcxsel(&self) -> RTCXSEL_R {
        RTCXSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Current setting of ddrctrlclksel mux"]
    #[inline(always)]
    pub fn ddrctrlclksel(&self) -> DDRCTRLCLKSEL_R {
        DDRCTRLCLKSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Current setting of ddrphyclksel mux"]
    #[inline(always)]
    pub fn ddrphyclksel(&self) -> DDRPHYCLKSEL_R {
        DDRPHYCLKSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Current setting of reserved0 mux"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Current setting of gemgxlclksel mux"]
    #[inline(always)]
    pub fn gemgxlclksel(&self) -> GEMGXLCLKSEL_R {
        GEMGXLCLKSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current setting of mainmemclksel mux"]
    #[inline(always)]
    pub fn mainmemclksel(&self) -> MAINMEMCLKSEL_R {
        MAINMEMCLKSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Current selection of each clock mux\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_mux_status](index.html) module"]
pub struct CLK_MUX_STATUS_SPEC;
impl crate::RegisterSpec for CLK_MUX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_mux_status::R](R) reader structure"]
impl crate::Readable for CLK_MUX_STATUS_SPEC {
    type Reader = R;
}
