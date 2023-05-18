#[doc = "Register `pwmcfg` reader"]
pub struct R(crate::R<PWMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwmcfg` writer"]
pub struct W(crate::W<PWMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCFG_SPEC>;
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
impl From<crate::W<PWMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwmscale` reader - PWM Counter scale"]
pub struct PWMSCALE_R(crate::FieldReader<u8, u8>);
impl PWMSCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWMSCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMSCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmscale` writer - PWM Counter scale"]
pub struct PWMSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `pwmsticky` reader - PWM Sticky - disallow clearing pwmcmpXip bits"]
pub struct PWMSTICKY_R(crate::FieldReader<bool, bool>);
impl PWMSTICKY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSTICKY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMSTICKY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmsticky` writer - PWM Sticky - disallow clearing pwmcmpXip bits"]
pub struct PWMSTICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSTICKY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `pwmzerocmp` reader - PWM Zero - counter resets to zero after match"]
pub struct PWMZEROCMP_R(crate::FieldReader<bool, bool>);
impl PWMZEROCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMZEROCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMZEROCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmzerocmp` writer - PWM Zero - counter resets to zero after match"]
pub struct PWMZEROCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMZEROCMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `pwmdeglitch` reader - PWM Deglitch - latch pwmcmpXip within same cycle"]
pub struct PWMDEGLITCH_R(crate::FieldReader<bool, bool>);
impl PWMDEGLITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMDEGLITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMDEGLITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmdeglitch` writer - PWM Deglitch - latch pwmcmpXip within same cycle"]
pub struct PWMDEGLITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMDEGLITCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `pwmenalways` reader - PWM enable always - run continuously"]
pub struct PWMENALWAYS_R(crate::FieldReader<bool, bool>);
impl PWMENALWAYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENALWAYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMENALWAYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmenalways` writer - PWM enable always - run continuously"]
pub struct PWMENALWAYS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENALWAYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `pwmenoneshot` reader - PWM enable one shot - run one cycle"]
pub struct PWMENONESHOT_R(crate::FieldReader<bool, bool>);
impl PWMENONESHOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENONESHOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMENONESHOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmenoneshot` writer - PWM enable one shot - run one cycle"]
pub struct PWMENONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENONESHOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `pwmcmp0center` reader - PWM0 Compare Center"]
pub struct PWMCMP0CENTER_R(crate::FieldReader<bool, bool>);
impl PWMCMP0CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP0CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP0CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp0center` writer - PWM0 Compare Center"]
pub struct PWMCMP0CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP0CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `pwmcmp1center` reader - PWM1 Compare Center"]
pub struct PWMCMP1CENTER_R(crate::FieldReader<bool, bool>);
impl PWMCMP1CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP1CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP1CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp1center` writer - PWM1 Compare Center"]
pub struct PWMCMP1CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP1CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `pwmcmp2center` reader - PWM2 Compare Center"]
pub struct PWMCMP2CENTER_R(crate::FieldReader<bool, bool>);
impl PWMCMP2CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP2CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP2CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp2center` writer - PWM2 Compare Center"]
pub struct PWMCMP2CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP2CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `pwmcmp3center` reader - PWM3 Compare Center"]
pub struct PWMCMP3CENTER_R(crate::FieldReader<bool, bool>);
impl PWMCMP3CENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP3CENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP3CENTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp3center` writer - PWM3 Compare Center"]
pub struct PWMCMP3CENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP3CENTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `pwmcmp0invert` reader - PWM0 Invert"]
pub struct PWMCMP0INVERT_R(crate::FieldReader<bool, bool>);
impl PWMCMP0INVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP0INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP0INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp0invert` writer - PWM0 Invert"]
pub struct PWMCMP0INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP0INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `pwmcmp1invert` reader - PWM1 Invert"]
pub struct PWMCMP1INVERT_R(crate::FieldReader<bool, bool>);
impl PWMCMP1INVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP1INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP1INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp1invert` writer - PWM1 Invert"]
pub struct PWMCMP1INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP1INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `pwmcmp2invert` reader - PWM2 Invert"]
pub struct PWMCMP2INVERT_R(crate::FieldReader<bool, bool>);
impl PWMCMP2INVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP2INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP2INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp2invert` writer - PWM2 Invert"]
pub struct PWMCMP2INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP2INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `pwmcmp3invert` reader - PWM3 Invert"]
pub struct PWMCMP3INVERT_R(crate::FieldReader<bool, bool>);
impl PWMCMP3INVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP3INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP3INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp3invert` writer - PWM3 Invert"]
pub struct PWMCMP3INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP3INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `pwmcmp0gang` reader - PWM0/PWM1 Compare Gang"]
pub struct PWMCMP0GANG_R(crate::FieldReader<bool, bool>);
impl PWMCMP0GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP0GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP0GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp0gang` writer - PWM0/PWM1 Compare Gang"]
pub struct PWMCMP0GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP0GANG_W<'a> {
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
#[doc = "Field `pwmcmp1gang` reader - PWM1/PWM2 Compare Gang"]
pub struct PWMCMP1GANG_R(crate::FieldReader<bool, bool>);
impl PWMCMP1GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP1GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP1GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp1gang` writer - PWM1/PWM2 Compare Gang"]
pub struct PWMCMP1GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP1GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `pwmcmp2gang` reader - PWM2/PWM3 Compare Gang"]
pub struct PWMCMP2GANG_R(crate::FieldReader<bool, bool>);
impl PWMCMP2GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP2GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP2GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp2gang` writer - PWM2/PWM3 Compare Gang"]
pub struct PWMCMP2GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP2GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `pwmcmp3gang` reader - PWM3/PWM0 Compare Gang"]
pub struct PWMCMP3GANG_R(crate::FieldReader<bool, bool>);
impl PWMCMP3GANG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP3GANG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP3GANG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp3gang` writer - PWM3/PWM0 Compare Gang"]
pub struct PWMCMP3GANG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP3GANG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `pwmcmp0ip` reader - PWM0 Interrupt Pending"]
pub struct PWMCMP0IP_R(crate::FieldReader<bool, bool>);
impl PWMCMP0IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP0IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP0IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp0ip` writer - PWM0 Interrupt Pending"]
pub struct PWMCMP0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP0IP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `pwmcmp1ip` reader - PWM1 Interrupt Pending"]
pub struct PWMCMP1IP_R(crate::FieldReader<bool, bool>);
impl PWMCMP1IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP1IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp1ip` writer - PWM1 Interrupt Pending"]
pub struct PWMCMP1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP1IP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `pwmcmp2ip` reader - PWM2 Interrupt Pending"]
pub struct PWMCMP2IP_R(crate::FieldReader<bool, bool>);
impl PWMCMP2IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP2IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp2ip` writer - PWM2 Interrupt Pending"]
pub struct PWMCMP2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP2IP_W<'a> {
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
#[doc = "Field `pwmcmp3ip` reader - PWM3 Interrupt Pending"]
pub struct PWMCMP3IP_R(crate::FieldReader<bool, bool>);
impl PWMCMP3IP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCMP3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCMP3IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwmcmp3ip` writer - PWM3 Interrupt Pending"]
pub struct PWMCMP3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCMP3IP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM Counter scale"]
    #[inline(always)]
    pub fn pwmscale(&self) -> PWMSCALE_R {
        PWMSCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWM Sticky - disallow clearing pwmcmpXip bits"]
    #[inline(always)]
    pub fn pwmsticky(&self) -> PWMSTICKY_R {
        PWMSTICKY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM Zero - counter resets to zero after match"]
    #[inline(always)]
    pub fn pwmzerocmp(&self) -> PWMZEROCMP_R {
        PWMZEROCMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM Deglitch - latch pwmcmpXip within same cycle"]
    #[inline(always)]
    pub fn pwmdeglitch(&self) -> PWMDEGLITCH_R {
        PWMDEGLITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM enable always - run continuously"]
    #[inline(always)]
    pub fn pwmenalways(&self) -> PWMENALWAYS_R {
        PWMENALWAYS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM enable one shot - run one cycle"]
    #[inline(always)]
    pub fn pwmenoneshot(&self) -> PWMENONESHOT_R {
        PWMENONESHOT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWM0 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp0center(&self) -> PWMCMP0CENTER_R {
        PWMCMP0CENTER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PWM1 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp1center(&self) -> PWMCMP1CENTER_R {
        PWMCMP1CENTER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PWM2 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp2center(&self) -> PWMCMP2CENTER_R {
        PWMCMP2CENTER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PWM3 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp3center(&self) -> PWMCMP3CENTER_R {
        PWMCMP3CENTER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWM0 Invert"]
    #[inline(always)]
    pub fn pwmcmp0invert(&self) -> PWMCMP0INVERT_R {
        PWMCMP0INVERT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PWM1 Invert"]
    #[inline(always)]
    pub fn pwmcmp1invert(&self) -> PWMCMP1INVERT_R {
        PWMCMP1INVERT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PWM2 Invert"]
    #[inline(always)]
    pub fn pwmcmp2invert(&self) -> PWMCMP2INVERT_R {
        PWMCMP2INVERT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PWM3 Invert"]
    #[inline(always)]
    pub fn pwmcmp3invert(&self) -> PWMCMP3INVERT_R {
        PWMCMP3INVERT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PWM0/PWM1 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp0gang(&self) -> PWMCMP0GANG_R {
        PWMCMP0GANG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PWM1/PWM2 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp1gang(&self) -> PWMCMP1GANG_R {
        PWMCMP1GANG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PWM2/PWM3 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp2gang(&self) -> PWMCMP2GANG_R {
        PWMCMP2GANG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PWM3/PWM0 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp3gang(&self) -> PWMCMP3GANG_R {
        PWMCMP3GANG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PWM0 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp0ip(&self) -> PWMCMP0IP_R {
        PWMCMP0IP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PWM1 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp1ip(&self) -> PWMCMP1IP_R {
        PWMCMP1IP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PWM2 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp2ip(&self) -> PWMCMP2IP_R {
        PWMCMP2IP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PWM3 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp3ip(&self) -> PWMCMP3IP_R {
        PWMCMP3IP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM Counter scale"]
    #[inline(always)]
    pub fn pwmscale(&mut self) -> PWMSCALE_W {
        PWMSCALE_W { w: self }
    }
    #[doc = "Bit 8 - PWM Sticky - disallow clearing pwmcmpXip bits"]
    #[inline(always)]
    pub fn pwmsticky(&mut self) -> PWMSTICKY_W {
        PWMSTICKY_W { w: self }
    }
    #[doc = "Bit 9 - PWM Zero - counter resets to zero after match"]
    #[inline(always)]
    pub fn pwmzerocmp(&mut self) -> PWMZEROCMP_W {
        PWMZEROCMP_W { w: self }
    }
    #[doc = "Bit 10 - PWM Deglitch - latch pwmcmpXip within same cycle"]
    #[inline(always)]
    pub fn pwmdeglitch(&mut self) -> PWMDEGLITCH_W {
        PWMDEGLITCH_W { w: self }
    }
    #[doc = "Bit 12 - PWM enable always - run continuously"]
    #[inline(always)]
    pub fn pwmenalways(&mut self) -> PWMENALWAYS_W {
        PWMENALWAYS_W { w: self }
    }
    #[doc = "Bit 13 - PWM enable one shot - run one cycle"]
    #[inline(always)]
    pub fn pwmenoneshot(&mut self) -> PWMENONESHOT_W {
        PWMENONESHOT_W { w: self }
    }
    #[doc = "Bit 16 - PWM0 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp0center(&mut self) -> PWMCMP0CENTER_W {
        PWMCMP0CENTER_W { w: self }
    }
    #[doc = "Bit 17 - PWM1 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp1center(&mut self) -> PWMCMP1CENTER_W {
        PWMCMP1CENTER_W { w: self }
    }
    #[doc = "Bit 18 - PWM2 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp2center(&mut self) -> PWMCMP2CENTER_W {
        PWMCMP2CENTER_W { w: self }
    }
    #[doc = "Bit 19 - PWM3 Compare Center"]
    #[inline(always)]
    pub fn pwmcmp3center(&mut self) -> PWMCMP3CENTER_W {
        PWMCMP3CENTER_W { w: self }
    }
    #[doc = "Bit 20 - PWM0 Invert"]
    #[inline(always)]
    pub fn pwmcmp0invert(&mut self) -> PWMCMP0INVERT_W {
        PWMCMP0INVERT_W { w: self }
    }
    #[doc = "Bit 21 - PWM1 Invert"]
    #[inline(always)]
    pub fn pwmcmp1invert(&mut self) -> PWMCMP1INVERT_W {
        PWMCMP1INVERT_W { w: self }
    }
    #[doc = "Bit 22 - PWM2 Invert"]
    #[inline(always)]
    pub fn pwmcmp2invert(&mut self) -> PWMCMP2INVERT_W {
        PWMCMP2INVERT_W { w: self }
    }
    #[doc = "Bit 23 - PWM3 Invert"]
    #[inline(always)]
    pub fn pwmcmp3invert(&mut self) -> PWMCMP3INVERT_W {
        PWMCMP3INVERT_W { w: self }
    }
    #[doc = "Bit 24 - PWM0/PWM1 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp0gang(&mut self) -> PWMCMP0GANG_W {
        PWMCMP0GANG_W { w: self }
    }
    #[doc = "Bit 25 - PWM1/PWM2 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp1gang(&mut self) -> PWMCMP1GANG_W {
        PWMCMP1GANG_W { w: self }
    }
    #[doc = "Bit 26 - PWM2/PWM3 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp2gang(&mut self) -> PWMCMP2GANG_W {
        PWMCMP2GANG_W { w: self }
    }
    #[doc = "Bit 27 - PWM3/PWM0 Compare Gang"]
    #[inline(always)]
    pub fn pwmcmp3gang(&mut self) -> PWMCMP3GANG_W {
        PWMCMP3GANG_W { w: self }
    }
    #[doc = "Bit 28 - PWM0 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp0ip(&mut self) -> PWMCMP0IP_W {
        PWMCMP0IP_W { w: self }
    }
    #[doc = "Bit 29 - PWM1 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp1ip(&mut self) -> PWMCMP1IP_W {
        PWMCMP1IP_W { w: self }
    }
    #[doc = "Bit 30 - PWM2 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp2ip(&mut self) -> PWMCMP2IP_W {
        PWMCMP2IP_W { w: self }
    }
    #[doc = "Bit 31 - PWM3 Interrupt Pending"]
    #[inline(always)]
    pub fn pwmcmp3ip(&mut self) -> PWMCMP3IP_W {
        PWMCMP3IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcfg](index.html) module"]
pub struct PWMCFG_SPEC;
impl crate::RegisterSpec for PWMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcfg::R](R) reader structure"]
impl crate::Readable for PWMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcfg::W](W) writer structure"]
impl crate::Writable for PWMCFG_SPEC {
    type Writer = W;
}
