#[doc = "Register `transmit__receive` reader"]
pub struct R(crate::R<TRANSMIT__RECEIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT__RECEIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT__RECEIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT__RECEIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `transmit__receive` writer"]
pub struct W(crate::W<TRANSMIT__RECEIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT__RECEIVE_SPEC>;
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
impl From<crate::W<TRANSMIT__RECEIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT__RECEIVE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit and receive data byte register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit__receive](index.html) module"]
pub struct TRANSMIT__RECEIVE_SPEC;
impl crate::RegisterSpec for TRANSMIT__RECEIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit__receive::R](R) reader structure"]
impl crate::Readable for TRANSMIT__RECEIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit__receive::W](W) writer structure"]
impl crate::Writable for TRANSMIT__RECEIVE_SPEC {
    type Writer = W;
}
