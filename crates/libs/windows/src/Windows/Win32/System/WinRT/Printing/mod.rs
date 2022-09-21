#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintManagerInterop(::windows::core::IUnknown);
impl IPrintManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrintManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IPrintManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintManagerInterop {}
impl ::core::fmt::Debug for IPrintManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5435a42_8d43_4e7b_a68a_ef311e392087);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowConfigurationNative(::windows::core::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn PrinterQueue(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrinterQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn DriverProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DriverProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub unsafe fn UserProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UserProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowConfigurationNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowConfigurationNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowConfigurationNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintWorkflowConfigurationNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowConfigurationNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowConfigurationNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintWorkflowConfigurationNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowConfigurationNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowConfigurationNative {}
impl ::core::fmt::Debug for IPrintWorkflowConfigurationNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowConfigurationNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc056be0a_9ee2_450a_9823_964f0006f2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub PrinterQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    PrinterQueue: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub DriverProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    DriverProperties: usize,
    #[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
    pub UserProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com")))]
    UserProperties: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(::windows::core::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    pub unsafe fn StartXpsOMGeneration<'a, P0>(&self, receiver: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IPrintWorkflowXpsReceiver>>,
    {
        (::windows::core::Interface::vtable(self).StartXpsOMGeneration)(::windows::core::Interface::as_raw(self), receiver.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn ObjectFactory(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ObjectFactory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::IXpsOMObjectFactory1>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowObjectModelSourceFileContentNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowObjectModelSourceFileContentNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowObjectModelSourceFileContentNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowObjectModelSourceFileContentNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowObjectModelSourceFileContentNative {}
impl ::core::fmt::Debug for IPrintWorkflowObjectModelSourceFileContentNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowObjectModelSourceFileContentNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68c9e477_993e_4052_8ac6_454eff58db9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartXpsOMGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub ObjectFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    ObjectFactory: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(::windows::core::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DocumentPackageTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>(result__)
    }
}
impl ::core::convert::From<IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowXpsObjectModelTargetPackageNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsObjectModelTargetPackageNative> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsObjectModelTargetPackageNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsObjectModelTargetPackageNative {}
impl ::core::fmt::Debug for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsObjectModelTargetPackageNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub DocumentPackageTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    DocumentPackageTarget: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver(::windows::core::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, P0>(&self, documentsequenceprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).SetDocumentSequencePrintTicket)(::windows::core::Interface::as_raw(self), documentsequenceprintticket.into().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<'a, P0>(&self, documentsequenceuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDocumentSequenceUri)(::windows::core::Interface::as_raw(self), documentsequenceuri.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, P0, P1>(&self, documentid: u32, documentprintticket: P0, documenturi: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDocumentData)(::windows::core::Interface::as_raw(self), documentid, documentprintticket.into().abi(), documenturi.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, P0, P1>(&self, documentid: u32, pageid: u32, pagereference: P0, pageuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPage)(::windows::core::Interface::as_raw(self), documentid, pageid, pagereference.into().abi(), pageuri.into()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowXpsReceiver> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintWorkflowXpsReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04097374_77b8_47f6_8167_aae29d4cf84b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentSequencePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentSequencePrintTicket: usize,
    pub SetDocumentSequenceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDocumentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: *mut ::core::ffi::c_void, documenturi: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDocumentData: usize,
    #[cfg(feature = "Win32_Storage_Xps")]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut ::core::ffi::c_void, pageuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))]
    AddPage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver2(::windows::core::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, P0>(&self, documentsequenceprintticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDocumentSequencePrintTicket)(::windows::core::Interface::as_raw(self), documentsequenceprintticket.into().abi()).ok()
    }
    pub unsafe fn SetDocumentSequenceUri<'a, P0>(&self, documentsequenceuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDocumentSequenceUri)(::windows::core::Interface::as_raw(self), documentsequenceuri.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDocumentData<'a, P0, P1>(&self, documentid: u32, documentprintticket: P0, documenturi: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDocumentData)(::windows::core::Interface::as_raw(self), documentid, documentprintticket.into().abi(), documenturi.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub unsafe fn AddPage<'a, P0, P1>(&self, documentid: u32, pageid: u32, pagereference: P0, pageuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Storage::Xps::IXpsOMPageReference>>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPage)(::windows::core::Interface::as_raw(self), documentid, pageid, pagereference.into().abi(), pageuri.into()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Failed(&self, xpserror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Failed)(::windows::core::Interface::as_raw(self), xpserror).ok()
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for ::windows::core::IUnknown {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowXpsReceiver2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for ::windows::core::IUnknown {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrintWorkflowXpsReceiver2> for &'a IPrintWorkflowXpsReceiver {
    fn from(value: &'a IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintWorkflowXpsReceiver2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintWorkflowXpsReceiver2 {}
impl ::core::fmt::Debug for IPrintWorkflowXpsReceiver2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintWorkflowXpsReceiver2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023bcc0c_dfab_4a61_b074_490c6995580d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_Vtbl {
    pub base__: IPrintWorkflowXpsReceiver_Vtbl,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpserror: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Printing\"`*"]
#[repr(transparent)]
pub struct IPrinting3DManagerInterop(::windows::core::IUnknown);
impl IPrinting3DManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ShowPrintUIForWindowAsync)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrinting3DManagerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPrinting3DManagerInterop> for ::windows::core::IInspectable {
    fn from(value: IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPrinting3DManagerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IPrinting3DManagerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrinting3DManagerInterop> for ::windows::core::IInspectable {
    fn from(value: &IPrinting3DManagerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPrinting3DManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrinting3DManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrinting3DManagerInterop {}
impl ::core::fmt::Debug for IPrinting3DManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrinting3DManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ca31010_1484_4587_b26b_dddf9f9caecd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowPrintUIForWindowAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
