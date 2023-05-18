#[doc = "Register `command__status` reader"]
pub struct R(crate::R<COMMAND__STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMAND__STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMAND__STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMAND__STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `command__status` writer"]
pub struct W(crate::W<COMMAND__STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND__STATUS_SPEC>;
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
impl From<crate::W<COMMAND__STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND__STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr_iack__rd_if` reader - Clear interrupt and Interrupt pending"]
pub struct WR_IACK__RD_IF_R(crate::FieldReader<bool, bool>);
impl WR_IACK__RD_IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_IACK__RD_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_IACK__RD_IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_iack__rd_if` writer - Clear interrupt and Interrupt pending"]
pub struct WR_IACK__RD_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_IACK__RD_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `wr_res__rd_tip` reader - Reserved and Transfer in progress"]
pub struct WR_RES__RD_TIP_R(crate::FieldReader<bool, bool>);
impl WR_RES__RD_TIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_RES__RD_TIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_RES__RD_TIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_res__rd_tip` writer - Reserved and Transfer in progress"]
pub struct WR_RES__RD_TIP_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RES__RD_TIP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `wr_res__rd_res` reader - Reserved and Reserved"]
pub struct WR_RES__RD_RES_R(crate::FieldReader<bool, bool>);
impl WR_RES__RD_RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_RES__RD_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_RES__RD_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_res__rd_res` writer - Reserved and Reserved"]
pub struct WR_RES__RD_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RES__RD_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `wr_ack__rd_res` reader - Send ACK/NACK and Reserved"]
pub struct WR_ACK__RD_RES_R(crate::FieldReader<bool, bool>);
impl WR_ACK__RD_RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_ACK__RD_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_ACK__RD_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_ack__rd_res` writer - Send ACK/NACK and Reserved"]
pub struct WR_ACK__RD_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_ACK__RD_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `wr_txd__rd_res` reader - Transmit data and Reserved"]
pub struct WR_TXD__RD_RES_R(crate::FieldReader<bool, bool>);
impl WR_TXD__RD_RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_TXD__RD_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_TXD__RD_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_txd__rd_res` writer - Transmit data and Reserved"]
pub struct WR_TXD__RD_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_TXD__RD_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `wr_rxd__rd_al` reader - Receive data and Arbitration lost"]
pub struct WR_RXD__RD_AL_R(crate::FieldReader<bool, bool>);
impl WR_RXD__RD_AL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_RXD__RD_AL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_RXD__RD_AL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_rxd__rd_al` writer - Receive data and Arbitration lost"]
pub struct WR_RXD__RD_AL_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_RXD__RD_AL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `wr_sto__rd_busy` reader - Generate stop and I2C bus busy"]
pub struct WR_STO__RD_BUSY_R(crate::FieldReader<bool, bool>);
impl WR_STO__RD_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_STO__RD_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_STO__RD_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_sto__rd_busy` writer - Generate stop and I2C bus busy"]
pub struct WR_STO__RD_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_STO__RD_BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `wr_sta__rd_rxack` reader - Generate start and Got ACK/NACK"]
pub struct WR_STA__RD_RXACK_R(crate::FieldReader<bool, bool>);
impl WR_STA__RD_RXACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_STA__RD_RXACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_STA__RD_RXACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr_sta__rd_rxack` writer - Generate start and Got ACK/NACK"]
pub struct WR_STA__RD_RXACK_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_STA__RD_RXACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear interrupt and Interrupt pending"]
    #[inline(always)]
    pub fn wr_iack__rd_if(&self) -> WR_IACK__RD_IF_R {
        WR_IACK__RD_IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reserved and Transfer in progress"]
    #[inline(always)]
    pub fn wr_res__rd_tip(&self) -> WR_RES__RD_TIP_R {
        WR_RES__RD_TIP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reserved and Reserved"]
    #[inline(always)]
    pub fn wr_res__rd_res(&self) -> WR_RES__RD_RES_R {
        WR_RES__RD_RES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Send ACK/NACK and Reserved"]
    #[inline(always)]
    pub fn wr_ack__rd_res(&self) -> WR_ACK__RD_RES_R {
        WR_ACK__RD_RES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit data and Reserved"]
    #[inline(always)]
    pub fn wr_txd__rd_res(&self) -> WR_TXD__RD_RES_R {
        WR_TXD__RD_RES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive data and Arbitration lost"]
    #[inline(always)]
    pub fn wr_rxd__rd_al(&self) -> WR_RXD__RD_AL_R {
        WR_RXD__RD_AL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generate stop and I2C bus busy"]
    #[inline(always)]
    pub fn wr_sto__rd_busy(&self) -> WR_STO__RD_BUSY_R {
        WR_STO__RD_BUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Generate start and Got ACK/NACK"]
    #[inline(always)]
    pub fn wr_sta__rd_rxack(&self) -> WR_STA__RD_RXACK_R {
        WR_STA__RD_RXACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear interrupt and Interrupt pending"]
    #[inline(always)]
    pub fn wr_iack__rd_if(&mut self) -> WR_IACK__RD_IF_W {
        WR_IACK__RD_IF_W { w: self }
    }
    #[doc = "Bit 1 - Reserved and Transfer in progress"]
    #[inline(always)]
    pub fn wr_res__rd_tip(&mut self) -> WR_RES__RD_TIP_W {
        WR_RES__RD_TIP_W { w: self }
    }
    #[doc = "Bit 2 - Reserved and Reserved"]
    #[inline(always)]
    pub fn wr_res__rd_res(&mut self) -> WR_RES__RD_RES_W {
        WR_RES__RD_RES_W { w: self }
    }
    #[doc = "Bit 3 - Send ACK/NACK and Reserved"]
    #[inline(always)]
    pub fn wr_ack__rd_res(&mut self) -> WR_ACK__RD_RES_W {
        WR_ACK__RD_RES_W { w: self }
    }
    #[doc = "Bit 4 - Transmit data and Reserved"]
    #[inline(always)]
    pub fn wr_txd__rd_res(&mut self) -> WR_TXD__RD_RES_W {
        WR_TXD__RD_RES_W { w: self }
    }
    #[doc = "Bit 5 - Receive data and Arbitration lost"]
    #[inline(always)]
    pub fn wr_rxd__rd_al(&mut self) -> WR_RXD__RD_AL_W {
        WR_RXD__RD_AL_W { w: self }
    }
    #[doc = "Bit 6 - Generate stop and I2C bus busy"]
    #[inline(always)]
    pub fn wr_sto__rd_busy(&mut self) -> WR_STO__RD_BUSY_W {
        WR_STO__RD_BUSY_W { w: self }
    }
    #[doc = "Bit 7 - Generate start and Got ACK/NACK"]
    #[inline(always)]
    pub fn wr_sta__rd_rxack(&mut self) -> WR_STA__RD_RXACK_W {
        WR_STA__RD_RXACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command write and status read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command__status](index.html) module"]
pub struct COMMAND__STATUS_SPEC;
impl crate::RegisterSpec for COMMAND__STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [command__status::R](R) reader structure"]
impl crate::Readable for COMMAND__STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command__status::W](W) writer structure"]
impl crate::Writable for COMMAND__STATUS_SPEC {
    type Writer = W;
}
