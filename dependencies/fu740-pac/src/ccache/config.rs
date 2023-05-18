#[doc = "Register `config` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `banks` reader - Number of banks in the cache"]
pub struct BANKS_R(crate::FieldReader<u8, u8>);
impl BANKS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ways` reader - Number of ways per bank"]
pub struct WAYS_R(crate::FieldReader<u8, u8>);
impl WAYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lgsets` reader - Base-2 logarithm of the sets per bank"]
pub struct LGSETS_R(crate::FieldReader<u8, u8>);
impl LGSETS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LGSETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LGSETS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lgblockbytes` reader - Base-2 logarithm of the bytes per cache block"]
pub struct LGBLOCKBYTES_R(crate::FieldReader<u8, u8>);
impl LGBLOCKBYTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        LGBLOCKBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LGBLOCKBYTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of banks in the cache"]
    #[inline(always)]
    pub fn banks(&self) -> BANKS_R {
        BANKS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of ways per bank"]
    #[inline(always)]
    pub fn ways(&self) -> WAYS_R {
        WAYS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Base-2 logarithm of the sets per bank"]
    #[inline(always)]
    pub fn lgsets(&self) -> LGSETS_R {
        LGSETS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Base-2 logarithm of the bytes per cache block"]
    #[inline(always)]
    pub fn lgblockbytes(&self) -> LGBLOCKBYTES_R {
        LGBLOCKBYTES_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Information about the Cache Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
