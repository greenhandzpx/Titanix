#[doc = "Register `out_xor` reader"]
pub struct R(crate::R<OUT_XOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_XOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_XOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_XOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `out_xor` writer"]
pub struct W(crate::W<OUT_XOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_XOR_SPEC>;
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
impl From<crate::W<OUT_XOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_XOR_SPEC>) -> Self {
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
#[doc = "Output XOR (invert)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_xor](index.html) module"]
pub struct OUT_XOR_SPEC;
impl crate::RegisterSpec for OUT_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_xor::R](R) reader structure"]
impl crate::Readable for OUT_XOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_xor::W](W) writer structure"]
impl crate::Writable for OUT_XOR_SPEC {
    type Writer = W;
}
