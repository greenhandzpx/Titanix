#[doc = "Register `input_val` reader"]
pub struct R(crate::R<INPUT_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `input_val` writer"]
pub struct W(crate::W<INPUT_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_VAL_SPEC>;
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
impl From<crate::W<INPUT_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUT_VAL_SPEC>) -> Self {
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
#[doc = "Pin value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input_val](index.html) module"]
pub struct INPUT_VAL_SPEC;
impl crate::RegisterSpec for INPUT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input_val::R](R) reader structure"]
impl crate::Readable for INPUT_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input_val::W](W) writer structure"]
impl crate::Writable for INPUT_VAL_SPEC {
    type Writer = W;
}
