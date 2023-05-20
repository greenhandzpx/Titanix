#[doc = "Register `prci_plls` reader"]
pub struct R(crate::R<PRCI_PLLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRCI_PLLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRCI_PLLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRCI_PLLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cltxpll` reader - Indicates presence of cltxpll"]
pub struct CLTXPLL_R(crate::FieldReader<bool, bool>);
impl CLTXPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLTXPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLTXPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gemgxlpll` reader - Indicates presence of gemgxlpll"]
pub struct GEMGXLPLL_R(crate::FieldReader<bool, bool>);
impl GEMGXLPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEMGXLPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEMGXLPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrpll` reader - Indicates presence of ddrpll"]
pub struct DDRPLL_R(crate::FieldReader<bool, bool>);
impl DDRPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hfpclkpll` reader - Indicates presence of hfpclkpll"]
pub struct HFPCLKPLL_R(crate::FieldReader<bool, bool>);
impl HFPCLKPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFPCLKPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFPCLKPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dvfscorepll` reader - Indicates presence of dvfscorepll"]
pub struct DVFSCOREPLL_R(crate::FieldReader<bool, bool>);
impl DVFSCOREPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DVFSCOREPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DVFSCOREPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `corepll` reader - Indicates presence of corepll"]
pub struct COREPLL_R(crate::FieldReader<bool, bool>);
impl COREPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COREPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates presence of cltxpll"]
    #[inline(always)]
    pub fn cltxpll(&self) -> CLTXPLL_R {
        CLTXPLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates presence of gemgxlpll"]
    #[inline(always)]
    pub fn gemgxlpll(&self) -> GEMGXLPLL_R {
        GEMGXLPLL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates presence of ddrpll"]
    #[inline(always)]
    pub fn ddrpll(&self) -> DDRPLL_R {
        DDRPLL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates presence of hfpclkpll"]
    #[inline(always)]
    pub fn hfpclkpll(&self) -> HFPCLKPLL_R {
        HFPCLKPLL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates presence of dvfscorepll"]
    #[inline(always)]
    pub fn dvfscorepll(&self) -> DVFSCOREPLL_R {
        DVFSCOREPLL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates presence of corepll"]
    #[inline(always)]
    pub fn corepll(&self) -> COREPLL_R {
        COREPLL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "PLL presence register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prci_plls](index.html) module"]
pub struct PRCI_PLLS_SPEC;
impl crate::RegisterSpec for PRCI_PLLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prci_plls::R](R) reader structure"]
impl crate::Readable for PRCI_PLLS_SPEC {
    type Reader = R;
}
