#[doc = "Register `FS_HCTSIZ4` reader"]
pub struct R(crate::R<FS_HCTSIZ4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_HCTSIZ4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_HCTSIZ4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_HCTSIZ4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_HCTSIZ4` writer"]
pub struct W(crate::W<FS_HCTSIZ4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_HCTSIZ4_SPEC>;
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
impl From<crate::W<FS_HCTSIZ4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_HCTSIZ4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_HCTSIZ4_SPEC, u32, u32, 19, O>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_HCTSIZ4_SPEC, u16, u16, 10, O>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_HCTSIZ4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<29> {
        DPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host channel-x transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz4](index.html) module"]
pub struct FS_HCTSIZ4_SPEC;
impl crate::RegisterSpec for FS_HCTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_hctsiz4::R](R) reader structure"]
impl crate::Readable for FS_HCTSIZ4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz4::W](W) writer structure"]
impl crate::Writable for FS_HCTSIZ4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_HCTSIZ4 to value 0"]
impl crate::Resettable for FS_HCTSIZ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
