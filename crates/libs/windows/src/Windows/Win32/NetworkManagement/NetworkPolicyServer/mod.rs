#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCOUNTINGPROPERTIES(pub i32);
pub const ALLOWEDIN8021X: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(64i32);
pub const ALLOWEDINCONDITION: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(4i32);
pub const ALLOWEDINPROFILE: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(2i32);
pub const ALLOWEDINPROXYCONDITION: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(16i32);
pub const ALLOWEDINPROXYPROFILE: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(8i32);
pub const ALLOWEDINVPNDIALUP: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTEFILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTEID(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTEINFO(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTEPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTERESTRICTIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTRIBUTESYNTAX(pub i32);
pub const ATTRIBUTE_FILTER_IEEE_802_1x: ATTRIBUTEFILTER = ATTRIBUTEFILTER(2i32);
pub const ATTRIBUTE_FILTER_NONE: ATTRIBUTEFILTER = ATTRIBUTEFILTER(0i32);
pub const ATTRIBUTE_FILTER_VPN_DIALUP: ATTRIBUTEFILTER = ATTRIBUTEFILTER(1i32);
pub const ATTRIBUTE_MIN_VALUE: ATTRIBUTEID = ATTRIBUTEID(1u32);
pub const ATTRIBUTE_UNDEFINED: ATTRIBUTEID = ATTRIBUTEID(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHENTICATION_TYPE(pub i32);
pub const AUTHSRV_AUTHORIZATION_VALUE_W: windows_core::PCWSTR = windows_core::w!("AuthorizationDLLs");
pub const AUTHSRV_ENFORCE_NP_FOR_PAP_CHALLENGE_RESPONSE_VALUE_W: windows_core::PCWSTR = windows_core::w!("EnforceNetworkPolicyForPAPBasedChallengeResponse");
pub const AUTHSRV_EXTENSIONS_VALUE_W: windows_core::PCWSTR = windows_core::w!("ExtensionDLLs");
pub const AUTHSRV_PARAMETERS_KEY_W: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\AuthSrv\\Parameters");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIENTPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONDITIONPROPERTIES(pub i32);
pub const DATA_STORE_DIRECTORY: IASDATASTORE = IASDATASTORE(1i32);
pub const DATA_STORE_LOCAL: IASDATASTORE = IASDATASTORE(0i32);
pub const DESCRIPTION: ATTRIBUTEINFO = ATTRIBUTEINFO(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DICTIONARYPROPERTIES(pub i32);
pub const DOMAIN_TYPE_MIXED: IASDOMAINTYPE = IASDOMAINTYPE(3i32);
pub const DOMAIN_TYPE_NONE: IASDOMAINTYPE = IASDOMAINTYPE(0i32);
pub const DOMAIN_TYPE_NT4: IASDOMAINTYPE = IASDOMAINTYPE(1i32);
pub const DOMAIN_TYPE_NT5: IASDOMAINTYPE = IASDOMAINTYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASCOMMONPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASCOMPONENTPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASDATASTORE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASDOMAINTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASOSTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASPROPERTIES(pub i32);
pub const IAS_ATTRIBUTE_ABSOLUTE_TIME: ATTRIBUTEID = ATTRIBUTEID(8149u32);
pub const IAS_ATTRIBUTE_ACCEPT_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(8167u32);
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4139u32);
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4138u32);
pub const IAS_ATTRIBUTE_ALLOWED_CERTIFICATE_EKU: ATTRIBUTEID = ATTRIBUTEID(4161u32);
pub const IAS_ATTRIBUTE_ALLOW_DIALIN: ATTRIBUTEID = ATTRIBUTEID(4111u32);
pub const IAS_ATTRIBUTE_AUTHENTICATION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4127u32);
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4137u32);
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4133u32);
pub const IAS_ATTRIBUTE_CERTIFICATE_EKU: ATTRIBUTEID = ATTRIBUTEID(8097u32);
pub const IAS_ATTRIBUTE_CERTIFICATE_THUMBPRINT: ATTRIBUTEID = ATTRIBUTEID(8250u32);
pub const IAS_ATTRIBUTE_CLEAR_TEXT_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(8107u32);
pub const IAS_ATTRIBUTE_CLIENT_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4108u32);
pub const IAS_ATTRIBUTE_CLIENT_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8139u32);
pub const IAS_ATTRIBUTE_CLIENT_NAME: ATTRIBUTEID = ATTRIBUTEID(4128u32);
pub const IAS_ATTRIBUTE_CLIENT_PACKET_HEADER: ATTRIBUTEID = ATTRIBUTEID(4109u32);
pub const IAS_ATTRIBUTE_CLIENT_QUARANTINE_COMPATIBLE: ATTRIBUTEID = ATTRIBUTEID(8131u32);
pub const IAS_ATTRIBUTE_CLIENT_UDP_PORT: ATTRIBUTEID = ATTRIBUTEID(4117u32);
pub const IAS_ATTRIBUTE_CLIENT_VENDOR_TYPE: ATTRIBUTEID = ATTRIBUTEID(4116u32);
pub const IAS_ATTRIBUTE_EAP_CONFIG: ATTRIBUTEID = ATTRIBUTEID(8098u32);
pub const IAS_ATTRIBUTE_EAP_FRIENDLY_NAME: ATTRIBUTEID = ATTRIBUTEID(4132u32);
pub const IAS_ATTRIBUTE_EAP_SESSION: ATTRIBUTEID = ATTRIBUTEID(8105u32);
pub const IAS_ATTRIBUTE_EAP_TYPEID: ATTRIBUTEID = ATTRIBUTEID(8101u32);
pub const IAS_ATTRIBUTE_EAP_TYPES_CONFIGURED_IN_PROXYPOLICY: ATTRIBUTEID = ATTRIBUTEID(8151u32);
pub const IAS_ATTRIBUTE_EXTENSION_STATE: ATTRIBUTEID = ATTRIBUTEID(4162u32);
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8129u32);
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(4130u32);
pub const IAS_ATTRIBUTE_GENERATE_CLASS_ATTRIBUTE: ATTRIBUTEID = ATTRIBUTEID(4158u32);
pub const IAS_ATTRIBUTE_GENERATE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4163u32);
pub const IAS_ATTRIBUTE_IGNORE_USER_DIALIN_PROPERTIES: ATTRIBUTEID = ATTRIBUTEID(4101u32);
pub const IAS_ATTRIBUTE_IS_REPLAY: ATTRIBUTEID = ATTRIBUTEID(8106u32);
pub const IAS_ATTRIBUTE_LOGGING_RESULT: ATTRIBUTEID = ATTRIBUTEID(8168u32);
pub const IAS_ATTRIBUTE_MACHINE_INVENTORY: ATTRIBUTEID = ATTRIBUTEID(8148u32);
pub const IAS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8126u32);
pub const IAS_ATTRIBUTE_MACHINE_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(8116u32);
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8118u32);
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_SID: ATTRIBUTEID = ATTRIBUTEID(8162u32);
pub const IAS_ATTRIBUTE_MACHINE_VALIDATED: ATTRIBUTEID = ATTRIBUTEID(8163u32);
pub const IAS_ATTRIBUTE_MANIPULATION_RULE: ATTRIBUTEID = ATTRIBUTEID(4114u32);
pub const IAS_ATTRIBUTE_MANIPULATION_TARGET: ATTRIBUTEID = ATTRIBUTEID(4113u32);
pub const IAS_ATTRIBUTE_NAME_MAPPED: ATTRIBUTEID = ATTRIBUTEID(8114u32);
pub const IAS_ATTRIBUTE_NP_ALLOWED_EAP_TYPE: ATTRIBUTEID = ATTRIBUTEID(4106u32);
pub const IAS_ATTRIBUTE_NP_ALLOWED_PORT_TYPES: ATTRIBUTEID = ATTRIBUTEID(4104u32);
pub const IAS_ATTRIBUTE_NP_AUTHENTICATION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4105u32);
pub const IAS_ATTRIBUTE_NP_CALLED_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4103u32);
pub const IAS_ATTRIBUTE_NP_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4098u32);
pub const IAS_ATTRIBUTE_NP_NAME: ATTRIBUTEID = ATTRIBUTEID(4149u32);
pub const IAS_ATTRIBUTE_NP_PEAPUPFRONT_ENABLED: ATTRIBUTEID = ATTRIBUTEID(4171u32);
pub const IAS_ATTRIBUTE_NP_TIME_OF_DAY: ATTRIBUTEID = ATTRIBUTEID(4102u32);
pub const IAS_ATTRIBUTE_NT4_ACCOUNT_NAME: ATTRIBUTEID = ATTRIBUTEID(4129u32);
pub const IAS_ATTRIBUTE_NT4_HCAP_ACCOUNT_NAME: ATTRIBUTEID = ATTRIBUTEID(8160u32);
pub const IAS_ATTRIBUTE_NT4_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8127u32);
pub const IAS_ATTRIBUTE_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(4131u32);
pub const IAS_ATTRIBUTE_ORIGINAL_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(4115u32);
pub const IAS_ATTRIBUTE_OVERRIDE_RAP_AUTH: ATTRIBUTEID = ATTRIBUTEID(8112u32);
pub const IAS_ATTRIBUTE_PACKET_TYPE: ATTRIBUTEID = ATTRIBUTEID(4136u32);
pub const IAS_ATTRIBUTE_PASSPORT_USER_MAPPING_UPN_SUFFIX: ATTRIBUTEID = ATTRIBUTEID(4169u32);
pub const IAS_ATTRIBUTE_PEAP_CHANNEL_UP: ATTRIBUTEID = ATTRIBUTEID(8113u32);
pub const IAS_ATTRIBUTE_PEAP_EMBEDDED_EAP_TYPEID: ATTRIBUTEID = ATTRIBUTEID(8099u32);
pub const IAS_ATTRIBUTE_PEAP_FAST_ROAMED_SESSION: ATTRIBUTEID = ATTRIBUTEID(8100u32);
pub const IAS_ATTRIBUTE_POLICY_ENFORCED: ATTRIBUTEID = ATTRIBUTEID(8115u32);
pub const IAS_ATTRIBUTE_POLICY_EVALUATED_SHV: ATTRIBUTEID = ATTRIBUTEID(8157u32);
pub const IAS_ATTRIBUTE_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4156u32);
pub const IAS_ATTRIBUTE_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4155u32);
pub const IAS_ATTRIBUTE_PROXY_EAP_CONFIG: ATTRIBUTEID = ATTRIBUTEID(8104u32);
pub const IAS_ATTRIBUTE_PROXY_POLICY_NAME: ATTRIBUTEID = ATTRIBUTEID(4154u32);
pub const IAS_ATTRIBUTE_PROXY_RETRY_COUNT: ATTRIBUTEID = ATTRIBUTEID(8147u32);
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8122u32);
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS_CONFIGURATION: ATTRIBUTEID = ATTRIBUTEID(8130u32);
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_HANDLE: ATTRIBUTEID = ATTRIBUTEID(8128u32);
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_ID: ATTRIBUTEID = ATTRIBUTEID(8133u32);
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_RESULT: ATTRIBUTEID = ATTRIBUTEID(8124u32);
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_VALIDATORS: ATTRIBUTEID = ATTRIBUTEID(8125u32);
pub const IAS_ATTRIBUTE_QUARANTINE_UPDATE_NON_COMPLIANT: ATTRIBUTEID = ATTRIBUTEID(8136u32);
pub const IAS_ATTRIBUTE_QUARANTINE_URL: ATTRIBUTEID = ATTRIBUTEID(8121u32);
pub const IAS_ATTRIBUTE_RADIUS_USERNAME_ENCODING_ASCII: ATTRIBUTEID = ATTRIBUTEID(8171u32);
pub const IAS_ATTRIBUTE_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(4142u32);
pub const IAS_ATTRIBUTE_REJECT_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(8103u32);
pub const IAS_ATTRIBUTE_REMOTE_RADIUS_TO_WINDOWS_USER_MAPPING: ATTRIBUTEID = ATTRIBUTEID(4168u32);
pub const IAS_ATTRIBUTE_REMOTE_SERVER_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4157u32);
pub const IAS_ATTRIBUTE_REQUEST_ID: ATTRIBUTEID = ATTRIBUTEID(4112u32);
pub const IAS_ATTRIBUTE_REQUEST_START_TIME: ATTRIBUTEID = ATTRIBUTEID(8137u32);
pub const IAS_ATTRIBUTE_SAVED_MACHINE_HEALTHCHECK_ONLY: ATTRIBUTEID = ATTRIBUTEID(8156u32);
pub const IAS_ATTRIBUTE_SAVED_NP_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4099u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_CALLBACK_NUMBER: ATTRIBUTEID = ATTRIBUTEID(4097u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_INTERFACE_ID: ATTRIBUTEID = ATTRIBUTEID(8140u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4096u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_PREFIX: ATTRIBUTEID = ATTRIBUTEID(8141u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_ROUTE: ATTRIBUTEID = ATTRIBUTEID(8142u32);
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_ROUTE: ATTRIBUTEID = ATTRIBUTEID(4100u32);
pub const IAS_ATTRIBUTE_SERVER_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8169u32);
pub const IAS_ATTRIBUTE_SERVER_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8170u32);
pub const IAS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4164u32);
pub const IAS_ATTRIBUTE_SHARED_SECRET: ATTRIBUTEID = ATTRIBUTEID(4107u32);
pub const IAS_ATTRIBUTE_SOH_CARRIER_EAPTLV: ATTRIBUTEID = ATTRIBUTEID(8154u32);
pub const IAS_ATTRIBUTE_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(4110u32);
pub const IAS_ATTRIBUTE_TUNNEL_TAG: ATTRIBUTEID = ATTRIBUTEID(4170u32);
pub const IAS_ATTRIBUTE_USER_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(8117u32);
pub const IAS_ATTRIBUTE_USER_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8119u32);
pub const IAS_ATTRIBUTE_USER_TOKEN_SID: ATTRIBUTEID = ATTRIBUTEID(8161u32);
pub const IAS_AUTH_ARAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(6i32);
pub const IAS_AUTH_CUSTOM: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(8i32);
pub const IAS_AUTH_EAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(5i32);
pub const IAS_AUTH_INVALID: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(0i32);
pub const IAS_AUTH_MD5CHAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(2i32);
pub const IAS_AUTH_MSCHAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(3i32);
pub const IAS_AUTH_MSCHAP2: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(4i32);
pub const IAS_AUTH_MSCHAP2_CPW: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(10i32);
pub const IAS_AUTH_MSCHAP_CPW: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(9i32);
pub const IAS_AUTH_NONE: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(7i32);
pub const IAS_AUTH_PAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(1i32);
pub const IAS_AUTH_PEAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(11i32);
pub const IAS_IDENTITY_NO_DEFAULT: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
pub const IAS_LOGGING_DAILY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(1i32);
pub const IAS_LOGGING_MONTHLY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(3i32);
pub const IAS_LOGGING_UNLIMITED_SIZE: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(0i32);
pub const IAS_LOGGING_WEEKLY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(2i32);
pub const IAS_LOGGING_WHEN_FILE_SIZE_REACHES: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(4i32);
pub const IAS_SYNTAX_BOOLEAN: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(1i32);
pub const IAS_SYNTAX_ENUMERATOR: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(3i32);
pub const IAS_SYNTAX_INETADDR: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(4i32);
pub const IAS_SYNTAX_INETADDR6: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(10i32);
pub const IAS_SYNTAX_INTEGER: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(2i32);
pub const IAS_SYNTAX_OCTETSTRING: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(6i32);
pub const IAS_SYNTAX_PROVIDERSPECIFIC: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(8i32);
pub const IAS_SYNTAX_STRING: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(5i32);
pub const IAS_SYNTAX_UNSIGNEDINTEGER: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(9i32);
pub const IAS_SYNTAX_UTCTIME: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDENTITY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPFILTERPROPERTIES(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdo, ISdo_Vtbl, 0x56bc53de_96db_11d1_bf3f_000000000000);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISdo {
    pub unsafe fn GetPropertyInfo(&self, id: i32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, id: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), id, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), id, core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn ResetProperty(&self, id: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetProperty)(windows_core::Interface::as_raw(self), id).ok() }
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Restore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutProperty: usize,
    pub ResetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdo_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetPropertyInfo(&self, id: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetProperty(&self, id: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ResetProperty(&self, id: i32) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdo_Vtbl {
    pub const fn new<Identity: ISdo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyInfo<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdo_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pppropertyinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdo_Impl::GetProperty(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutProperty<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdo_Impl::PutProperty(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn ResetProperty<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdo_Impl::ResetProperty(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdo_Impl::Apply(this).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdo_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdo_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
            ResetProperty: ResetProperty::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdoCollection, ISdoCollection_Vtbl, 0x56bc53e2_96db_11d1_bf3f_000000000000);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdoCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdoCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISdoCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, bstrname: &windows_core::BSTR, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute(ppitem)).ok() }
    }
    pub unsafe fn Remove<P0>(&self, pitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), pitem.param().abi()).ok() }
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reload(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reload)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsNameUnique(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNameUnique)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Item(&self, name: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsNameUnique: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrname: &windows_core::BSTR, ppitem: windows_core::OutRef<'_, super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn Remove(&self, pitem: windows_core::Ref<'_, super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn Reload(&self) -> windows_core::Result<()>;
    fn IsNameUnique(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Item(&self, name: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoCollection_Vtbl {
    pub const fn new<Identity: ISdoCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoCollection_Impl::Add(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoCollection_Impl::Remove(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        unsafe extern "system" fn RemoveAll<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoCollection_Impl::RemoveAll(this).into()
            }
        }
        unsafe extern "system" fn Reload<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoCollection_Impl::Reload(this).into()
            }
        }
        unsafe extern "system" fn IsNameUnique<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoCollection_Impl::IsNameUnique(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbool.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const super::super::System::Variant::VARIANT, pitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoCollection_Impl::Item(this, core::mem::transmute_copy(&name)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISdoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            Reload: Reload::<Identity, OFFSET>,
            IsNameUnique: IsNameUnique::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdoCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdoDictionaryOld, ISdoDictionaryOld_Vtbl, 0xd432e5f4_53d8_11d2_9a3a_00c04fb998ac);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdoDictionaryOld {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdoDictionaryOld, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISdoDictionaryOld {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnumAttributes(&self, id: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAttributes)(windows_core::Interface::as_raw(self), core::mem::transmute(id), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeInfo)(windows_core::Interface::as_raw(self), id, core::mem::transmute(pinfoids), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAttributeValues)(windows_core::Interface::as_raw(self), id, core::mem::transmute(pvalueids), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateAttribute(&self, id: ATTRIBUTEID) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAttribute)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAttributeID(&self, bstrattributename: &windows_core::BSTR) -> windows_core::Result<ATTRIBUTEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattributename), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoDictionaryOld_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnumAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnumAttributes: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, ATTRIBUTEID, *const super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttributeInfo: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnumAttributeValues: unsafe extern "system" fn(*mut core::ffi::c_void, ATTRIBUTEID, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnumAttributeValues: usize,
    pub CreateAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, ATTRIBUTEID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut ATTRIBUTEID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoDictionaryOld_Impl: super::super::System::Com::IDispatch_Impl {
    fn EnumAttributes(&self, id: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CreateAttribute(&self, id: ATTRIBUTEID) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetAttributeID(&self, bstrattributename: &windows_core::BSTR) -> windows_core::Result<ATTRIBUTEID>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoDictionaryOld_Vtbl {
    pub const fn new<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAttributes<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::super::System::Variant::VARIANT, pvalues: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoDictionaryOld_Impl::EnumAttributes(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT, pinfovalues: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoDictionaryOld_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pinfoids)) {
                    Ok(ok__) => {
                        pinfovalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumAttributeValues<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT, pvaluesdesc: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoDictionaryOld_Impl::EnumAttributeValues(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pvalueids)) {
                    Ok(ok__) => {
                        pvaluesdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAttribute<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoDictionaryOld_Impl::CreateAttribute(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        ppattributeobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeID<Identity: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattributename: *mut core::ffi::c_void, pid: *mut ATTRIBUTEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoDictionaryOld_Impl::GetAttributeID(this, core::mem::transmute(&bstrattributename)) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET>,
            EnumAttributeValues: EnumAttributeValues::<Identity, OFFSET>,
            CreateAttribute: CreateAttribute::<Identity, OFFSET>,
            GetAttributeID: GetAttributeID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoDictionaryOld as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdoDictionaryOld {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdoMachine, ISdoMachine_Vtbl, 0x479f6e75_49a2_11d2_8eca_00c04fc2f519);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdoMachine {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdoMachine, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine {
    pub unsafe fn Attach(&self, bstrcomputername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcomputername)).ok() }
    }
    pub unsafe fn GetDictionarySDO(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionarySDO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetServiceSDO(&self, edatastore: IASDATASTORE, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceSDO)(windows_core::Interface::as_raw(self), edatastore, core::mem::transmute_copy(bstrservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUserSDO(&self, edatastore: IASDATASTORE, bstrusername: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSDO)(windows_core::Interface::as_raw(self), edatastore, core::mem::transmute_copy(bstrusername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOSType(&self) -> windows_core::Result<IASOSTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOSType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDomainType(&self) -> windows_core::Result<IASDOMAINTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomainType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDirectoryAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDirectoryAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAttachedComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttachedComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSDOSchema(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSDOSchema)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoMachine_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDictionarySDO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceSDO: unsafe extern "system" fn(*mut core::ffi::c_void, IASDATASTORE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserSDO: unsafe extern "system" fn(*mut core::ffi::c_void, IASDATASTORE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOSType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IASOSTYPE) -> windows_core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IASDOMAINTYPE) -> windows_core::HRESULT,
    pub IsDirectoryAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetAttachedComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSDOSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine_Impl: super::super::System::Com::IDispatch_Impl {
    fn Attach(&self, bstrcomputername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDictionarySDO(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetServiceSDO(&self, edatastore: IASDATASTORE, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetUserSDO(&self, edatastore: IASDATASTORE, bstrusername: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetOSType(&self) -> windows_core::Result<IASOSTYPE>;
    fn GetDomainType(&self) -> windows_core::Result<IASDOMAINTYPE>;
    fn IsDirectoryAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAttachedComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSDOSchema(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoMachine_Vtbl {
    pub const fn new<Identity: ISdoMachine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Attach<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcomputername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoMachine_Impl::Attach(this, core::mem::transmute(&bstrcomputername)).into()
            }
        }
        unsafe extern "system" fn GetDictionarySDO<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdictionarysdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetDictionarySDO(this) {
                    Ok(ok__) => {
                        ppdictionarysdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceSDO<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: *mut core::ffi::c_void, ppservicesdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetServiceSDO(this, core::mem::transmute_copy(&edatastore), core::mem::transmute(&bstrservicename)) {
                    Ok(ok__) => {
                        ppservicesdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserSDO<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: *mut core::ffi::c_void, ppusersdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetUserSDO(this, core::mem::transmute_copy(&edatastore), core::mem::transmute(&bstrusername)) {
                    Ok(ok__) => {
                        ppusersdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOSType<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eostype: *mut IASOSTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetOSType(this) {
                    Ok(ok__) => {
                        eostype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetDomainType(this) {
                    Ok(ok__) => {
                        edomaintype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDirectoryAvailable<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, booldirectoryavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::IsDirectoryAvailable(this) {
                    Ok(ok__) => {
                        booldirectoryavailable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttachedComputer<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcomputername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetAttachedComputer(this) {
                    Ok(ok__) => {
                        bstrcomputername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSDOSchema<Identity: ISdoMachine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsdoschema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine_Impl::GetSDOSchema(this) {
                    Ok(ok__) => {
                        ppsdoschema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Attach: Attach::<Identity, OFFSET>,
            GetDictionarySDO: GetDictionarySDO::<Identity, OFFSET>,
            GetServiceSDO: GetServiceSDO::<Identity, OFFSET>,
            GetUserSDO: GetUserSDO::<Identity, OFFSET>,
            GetOSType: GetOSType::<Identity, OFFSET>,
            GetDomainType: GetDomainType::<Identity, OFFSET>,
            IsDirectoryAvailable: IsDirectoryAvailable::<Identity, OFFSET>,
            GetAttachedComputer: GetAttachedComputer::<Identity, OFFSET>,
            GetSDOSchema: GetSDOSchema::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoMachine as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdoMachine {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdoMachine2, ISdoMachine2_Vtbl, 0x518e5ffe_d8ce_4f7e_a5db_b40a35419d3b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdoMachine2 {
    type Target = ISdoMachine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdoMachine2, windows_core::IUnknown, super::super::System::Com::IDispatch, ISdoMachine);
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine2 {
    pub unsafe fn GetTemplatesSDO(&self, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemplatesSDO)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableTemplates(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableTemplates)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SyncConfigAgainstTemplates(&self, bstrservicename: &windows_core::BSTR, ppconfigroot: *mut Option<windows_core::IUnknown>, pptemplatesroot: *mut Option<windows_core::IUnknown>, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SyncConfigAgainstTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservicename), core::mem::transmute(ppconfigroot), core::mem::transmute(pptemplatesroot), bforcedsync).ok() }
    }
    pub unsafe fn ImportRemoteTemplates<P0>(&self, plocaltemplatesroot: P0, bstrremotemachinename: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ImportRemoteTemplates)(windows_core::Interface::as_raw(self), plocaltemplatesroot.param().abi(), core::mem::transmute_copy(bstrremotemachinename)).ok() }
    }
    pub unsafe fn Reload(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reload)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoMachine2_Vtbl {
    pub base__: ISdoMachine_Vtbl,
    pub GetTemplatesSDO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableTemplates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncConfigAgainstTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ImportRemoteTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine2_Impl: ISdoMachine_Impl {
    fn GetTemplatesSDO(&self, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn EnableTemplates(&self) -> windows_core::Result<()>;
    fn SyncConfigAgainstTemplates(&self, bstrservicename: &windows_core::BSTR, ppconfigroot: windows_core::OutRef<'_, windows_core::IUnknown>, pptemplatesroot: windows_core::OutRef<'_, windows_core::IUnknown>, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ImportRemoteTemplates(&self, plocaltemplatesroot: windows_core::Ref<'_, windows_core::IUnknown>, bstrremotemachinename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Reload(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoMachine2_Vtbl {
    pub const fn new<Identity: ISdoMachine2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTemplatesSDO<Identity: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservicename: *mut core::ffi::c_void, pptemplatessdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoMachine2_Impl::GetTemplatesSDO(this, core::mem::transmute(&bstrservicename)) {
                    Ok(ok__) => {
                        pptemplatessdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableTemplates<Identity: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoMachine2_Impl::EnableTemplates(this).into()
            }
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Identity: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservicename: *mut core::ffi::c_void, ppconfigroot: *mut *mut core::ffi::c_void, pptemplatesroot: *mut *mut core::ffi::c_void, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoMachine2_Impl::SyncConfigAgainstTemplates(this, core::mem::transmute(&bstrservicename), core::mem::transmute_copy(&ppconfigroot), core::mem::transmute_copy(&pptemplatesroot), core::mem::transmute_copy(&bforcedsync)).into()
            }
        }
        unsafe extern "system" fn ImportRemoteTemplates<Identity: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocaltemplatesroot: *mut core::ffi::c_void, bstrremotemachinename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoMachine2_Impl::ImportRemoteTemplates(this, core::mem::transmute_copy(&plocaltemplatesroot), core::mem::transmute(&bstrremotemachinename)).into()
            }
        }
        unsafe extern "system" fn Reload<Identity: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoMachine2_Impl::Reload(this).into()
            }
        }
        Self {
            base__: ISdoMachine_Vtbl::new::<Identity, OFFSET>(),
            GetTemplatesSDO: GetTemplatesSDO::<Identity, OFFSET>,
            EnableTemplates: EnableTemplates::<Identity, OFFSET>,
            SyncConfigAgainstTemplates: SyncConfigAgainstTemplates::<Identity, OFFSET>,
            ImportRemoteTemplates: ImportRemoteTemplates::<Identity, OFFSET>,
            Reload: Reload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoMachine2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISdoMachine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdoMachine2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISdoServiceControl, ISdoServiceControl_Vtbl, 0x479f6e74_49a2_11d2_8eca_00c04fc2f519);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISdoServiceControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISdoServiceControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISdoServiceControl {
    pub unsafe fn StartService(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartService)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StopService(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopService)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetServiceStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResetService(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetService)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoServiceControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResetService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoServiceControl_Impl: super::super::System::Com::IDispatch_Impl {
    fn StartService(&self) -> windows_core::Result<()>;
    fn StopService(&self) -> windows_core::Result<()>;
    fn GetServiceStatus(&self) -> windows_core::Result<i32>;
    fn ResetService(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoServiceControl_Vtbl {
    pub const fn new<Identity: ISdoServiceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartService<Identity: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoServiceControl_Impl::StartService(this).into()
            }
        }
        unsafe extern "system" fn StopService<Identity: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoServiceControl_Impl::StopService(this).into()
            }
        }
        unsafe extern "system" fn GetServiceStatus<Identity: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISdoServiceControl_Impl::GetServiceStatus(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetService<Identity: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISdoServiceControl_Impl::ResetService(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartService: StartService::<Identity, OFFSET>,
            StopService: StopService::<Identity, OFFSET>,
            GetServiceStatus: GetServiceStatus::<Identity, OFFSET>,
            ResetService: ResetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoServiceControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISdoServiceControl {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITemplateSdo, ITemplateSdo_Vtbl, 0x8aa85302_d2e2_4e20_8b1f_a571e437d6c9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITemplateSdo {
    type Target = ISdo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITemplateSdo, windows_core::IUnknown, super::super::System::Com::IDispatch, ISdo);
#[cfg(feature = "Win32_System_Com")]
impl ITemplateSdo {
    pub unsafe fn AddToCollection<P1>(&self, bstrname: &windows_core::BSTR, pcollection: P1, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), pcollection.param().abi(), core::mem::transmute(ppitem)).ok() }
    }
    pub unsafe fn AddToSdo<P1>(&self, bstrname: &windows_core::BSTR, psdotarget: P1, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToSdo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), psdotarget.param().abi(), core::mem::transmute(ppitem)).ok() }
    }
    pub unsafe fn AddToSdoAsProperty<P0>(&self, psdotarget: P0, id: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToSdoAsProperty)(windows_core::Interface::as_raw(self), psdotarget.param().abi(), id).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITemplateSdo_Vtbl {
    pub base__: ISdo_Vtbl,
    pub AddToCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSdo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSdoAsProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITemplateSdo_Impl: ISdo_Impl {
    fn AddToCollection(&self, bstrname: &windows_core::BSTR, pcollection: windows_core::Ref<'_, super::super::System::Com::IDispatch>, ppitem: windows_core::OutRef<'_, super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn AddToSdo(&self, bstrname: &windows_core::BSTR, psdotarget: windows_core::Ref<'_, super::super::System::Com::IDispatch>, ppitem: windows_core::OutRef<'_, super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn AddToSdoAsProperty(&self, psdotarget: windows_core::Ref<'_, super::super::System::Com::IDispatch>, id: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITemplateSdo_Vtbl {
    pub const fn new<Identity: ITemplateSdo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddToCollection<Identity: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pcollection: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITemplateSdo_Impl::AddToCollection(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&pcollection), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn AddToSdo<Identity: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, psdotarget: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITemplateSdo_Impl::AddToSdo(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&psdotarget), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn AddToSdoAsProperty<Identity: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psdotarget: *mut core::ffi::c_void, id: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITemplateSdo_Impl::AddToSdoAsProperty(this, core::mem::transmute_copy(&psdotarget), core::mem::transmute_copy(&id)).into()
            }
        }
        Self {
            base__: ISdo_Vtbl::new::<Identity, OFFSET>(),
            AddToCollection: AddToCollection::<Identity, OFFSET>,
            AddToSdo: AddToSdo::<Identity, OFFSET>,
            AddToSdoAsProperty: AddToSdoAsProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITemplateSdo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISdo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITemplateSdo {}
pub const LDAPNAME: ATTRIBUTEINFO = ATTRIBUTEINFO(6i32);
pub const MS_ATTRIBUTE_ACCT_AUTH_TYPE: ATTRIBUTEID = ATTRIBUTEID(4134u32);
pub const MS_ATTRIBUTE_ACCT_EAP_TYPE: ATTRIBUTEID = ATTRIBUTEID(4135u32);
pub const MS_ATTRIBUTE_AFW_PROTECTION_LEVEL: ATTRIBUTEID = ATTRIBUTEID(8135u32);
pub const MS_ATTRIBUTE_AFW_QUARANTINE_ZONE: ATTRIBUTEID = ATTRIBUTEID(8134u32);
pub const MS_ATTRIBUTE_AZURE_POLICY_ID: ATTRIBUTEID = ATTRIBUTEID(8173u32);
pub const MS_ATTRIBUTE_CHAP2_CPW: ATTRIBUTEID = ATTRIBUTEID(4146u32);
pub const MS_ATTRIBUTE_CHAP2_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(4144u32);
pub const MS_ATTRIBUTE_CHAP2_SUCCESS: ATTRIBUTEID = ATTRIBUTEID(4145u32);
pub const MS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = ATTRIBUTEID(4118u32);
pub const MS_ATTRIBUTE_CHAP_CPW1: ATTRIBUTEID = ATTRIBUTEID(4122u32);
pub const MS_ATTRIBUTE_CHAP_CPW2: ATTRIBUTEID = ATTRIBUTEID(4123u32);
pub const MS_ATTRIBUTE_CHAP_DOMAIN: ATTRIBUTEID = ATTRIBUTEID(4120u32);
pub const MS_ATTRIBUTE_CHAP_ERROR: ATTRIBUTEID = ATTRIBUTEID(4121u32);
pub const MS_ATTRIBUTE_CHAP_LM_ENC_PW: ATTRIBUTEID = ATTRIBUTEID(4124u32);
pub const MS_ATTRIBUTE_CHAP_MPPE_KEYS: ATTRIBUTEID = ATTRIBUTEID(4126u32);
pub const MS_ATTRIBUTE_CHAP_NT_ENC_PW: ATTRIBUTEID = ATTRIBUTEID(4125u32);
pub const MS_ATTRIBUTE_CHAP_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(4119u32);
pub const MS_ATTRIBUTE_EAP_TLV: ATTRIBUTEID = ATTRIBUTEID(8102u32);
pub const MS_ATTRIBUTE_EXTENDED_QUARANTINE_STATE: ATTRIBUTEID = ATTRIBUTEID(8153u32);
pub const MS_ATTRIBUTE_FILTER: ATTRIBUTEID = ATTRIBUTEID(4143u32);
pub const MS_ATTRIBUTE_HCAP_LOCATION_GROUP_NAME: ATTRIBUTEID = ATTRIBUTEID(8152u32);
pub const MS_ATTRIBUTE_HCAP_USER_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8155u32);
pub const MS_ATTRIBUTE_HCAP_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(8159u32);
pub const MS_ATTRIBUTE_IDENTITY_TYPE: ATTRIBUTEID = ATTRIBUTEID(8108u32);
pub const MS_ATTRIBUTE_IPV4_REMEDIATION_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8145u32);
pub const MS_ATTRIBUTE_IPV6_REMEDIATION_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8146u32);
pub const MS_ATTRIBUTE_IPv6_FILTER: ATTRIBUTEID = ATTRIBUTEID(8144u32);
pub const MS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8138u32);
pub const MS_ATTRIBUTE_MPPE_RECV_KEY: ATTRIBUTEID = ATTRIBUTEID(4141u32);
pub const MS_ATTRIBUTE_MPPE_SEND_KEY: ATTRIBUTEID = ATTRIBUTEID(4140u32);
pub const MS_ATTRIBUTE_NETWORK_ACCESS_SERVER_TYPE: ATTRIBUTEID = ATTRIBUTEID(8132u32);
pub const MS_ATTRIBUTE_NOT_QUARANTINE_CAPABLE: ATTRIBUTEID = ATTRIBUTEID(8123u32);
pub const MS_ATTRIBUTE_PRIMARY_DNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4150u32);
pub const MS_ATTRIBUTE_PRIMARY_NBNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4152u32);
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME: ATTRIBUTEID = ATTRIBUTEID(8120u32);
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME_CONFIGURATION: ATTRIBUTEID = ATTRIBUTEID(8143u32);
pub const MS_ATTRIBUTE_QUARANTINE_IPFILTER: ATTRIBUTEID = ATTRIBUTEID(4165u32);
pub const MS_ATTRIBUTE_QUARANTINE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4166u32);
pub const MS_ATTRIBUTE_QUARANTINE_SOH: ATTRIBUTEID = ATTRIBUTEID(8150u32);
pub const MS_ATTRIBUTE_QUARANTINE_STATE: ATTRIBUTEID = ATTRIBUTEID(8111u32);
pub const MS_ATTRIBUTE_QUARANTINE_USER_CLASS: ATTRIBUTEID = ATTRIBUTEID(8110u32);
pub const MS_ATTRIBUTE_RAS_CLIENT_NAME: ATTRIBUTEID = ATTRIBUTEID(4159u32);
pub const MS_ATTRIBUTE_RAS_CLIENT_VERSION: ATTRIBUTEID = ATTRIBUTEID(4160u32);
pub const MS_ATTRIBUTE_RAS_CORRELATION_ID: ATTRIBUTEID = ATTRIBUTEID(8158u32);
pub const MS_ATTRIBUTE_RAS_ROUTING_DOMAIN_ID: ATTRIBUTEID = ATTRIBUTEID(8172u32);
pub const MS_ATTRIBUTE_RAS_VENDOR: ATTRIBUTEID = ATTRIBUTEID(4147u32);
pub const MS_ATTRIBUTE_RAS_VERSION: ATTRIBUTEID = ATTRIBUTEID(4148u32);
pub const MS_ATTRIBUTE_SECONDARY_DNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4151u32);
pub const MS_ATTRIBUTE_SECONDARY_NBNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4153u32);
pub const MS_ATTRIBUTE_SERVICE_CLASS: ATTRIBUTEID = ATTRIBUTEID(8109u32);
pub const MS_ATTRIBUTE_TSG_DEVICE_REDIRECTION: ATTRIBUTEID = ATTRIBUTEID(8166u32);
pub const MS_ATTRIBUTE_USER_IPv4_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8164u32);
pub const MS_ATTRIBUTE_USER_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8165u32);
pub const MS_ATTRIBUTE_USER_SECURITY_IDENTITY: ATTRIBUTEID = ATTRIBUTEID(4167u32);
pub const MULTIVALUED: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(1i32);
pub const NAME: ATTRIBUTEINFO = ATTRIBUTEINFO(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAMESPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAPPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NEW_LOG_FILE_FREQUENCY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTEVENTLOGPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NTSAMPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICYPROPERTIES(pub i32);
pub type PRADIUS_EXTENSION_FREE_ATTRIBUTES = Option<unsafe extern "system" fn(pattrs: *mut RADIUS_ATTRIBUTE)>;
pub type PRADIUS_EXTENSION_INIT = Option<unsafe extern "system" fn() -> u32>;
pub type PRADIUS_EXTENSION_PROCESS = Option<unsafe extern "system" fn(pattrs: *const RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
pub type PRADIUS_EXTENSION_PROCESS_2 = Option<unsafe extern "system" fn(pecb: *mut RADIUS_EXTENSION_CONTROL_BLOCK) -> u32>;
pub type PRADIUS_EXTENSION_PROCESS_EX = Option<unsafe extern "system" fn(pinattrs: *const RADIUS_ATTRIBUTE, poutattrs: *mut *mut RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
pub type PRADIUS_EXTENSION_TERM = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILEPROPERTIES(pub i32);
pub const PROPERTY_ACCOUNTING_DISCARD_REQUEST_ON_FAILURE: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1038i32);
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1026i32);
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING_INTERIM: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1027i32);
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1028i32);
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION_INTERIM: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1036i32);
pub const PROPERTY_ACCOUNTING_LOG_DELETE_IF_FULL: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1034i32);
pub const PROPERTY_ACCOUNTING_LOG_ENABLE_LOGGING: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1033i32);
pub const PROPERTY_ACCOUNTING_LOG_FILE_DIRECTORY: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1031i32);
pub const PROPERTY_ACCOUNTING_LOG_FILE_IS_BACKUP: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1037i32);
pub const PROPERTY_ACCOUNTING_LOG_IAS1_FORMAT: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1032i32);
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_FREQUENCY: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1029i32);
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_SIZE: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1030i32);
pub const PROPERTY_ACCOUNTING_SQL_MAX_SESSIONS: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1035i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_8021X: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1040i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_CONDITION: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1034i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROFILE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1033i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_CONDITION: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1038i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_PROFILE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1037i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_VPNDIALUP: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1039i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_LOG_ORDINAL: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1032i32);
pub const PROPERTY_ATTRIBUTE_ALLOW_MULTIPLE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1031i32);
pub const PROPERTY_ATTRIBUTE_DISPLAY_NAME: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1035i32);
pub const PROPERTY_ATTRIBUTE_ENUM_FILTERS: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1041i32);
pub const PROPERTY_ATTRIBUTE_ENUM_NAMES: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1028i32);
pub const PROPERTY_ATTRIBUTE_ENUM_VALUES: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1029i32);
pub const PROPERTY_ATTRIBUTE_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1024i32);
pub const PROPERTY_ATTRIBUTE_IS_ENUMERABLE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1027i32);
pub const PROPERTY_ATTRIBUTE_SYNTAX: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1030i32);
pub const PROPERTY_ATTRIBUTE_VALUE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1036i32);
pub const PROPERTY_ATTRIBUTE_VENDOR_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1025i32);
pub const PROPERTY_ATTRIBUTE_VENDOR_TYPE_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1026i32);
pub const PROPERTY_CLIENT_ADDRESS: CLIENTPROPERTIES = CLIENTPROPERTIES(1028i32);
pub const PROPERTY_CLIENT_ENABLED: CLIENTPROPERTIES = CLIENTPROPERTIES(1030i32);
pub const PROPERTY_CLIENT_NAS_MANUFACTURER: CLIENTPROPERTIES = CLIENTPROPERTIES(1027i32);
pub const PROPERTY_CLIENT_QUARANTINE_COMPATIBLE: CLIENTPROPERTIES = CLIENTPROPERTIES(1029i32);
pub const PROPERTY_CLIENT_REQUIRE_SIGNATURE: CLIENTPROPERTIES = CLIENTPROPERTIES(1024i32);
pub const PROPERTY_CLIENT_SECRET_TEMPLATE_GUID: CLIENTPROPERTIES = CLIENTPROPERTIES(1031i32);
pub const PROPERTY_CLIENT_SHARED_SECRET: CLIENTPROPERTIES = CLIENTPROPERTIES(1026i32);
pub const PROPERTY_CLIENT_UNUSED: CLIENTPROPERTIES = CLIENTPROPERTIES(1025i32);
pub const PROPERTY_COMPONENT_ID: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1024i32);
pub const PROPERTY_COMPONENT_PROG_ID: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1025i32);
pub const PROPERTY_COMPONENT_START: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1026i32);
pub const PROPERTY_CONDITION_TEXT: CONDITIONPROPERTIES = CONDITIONPROPERTIES(1024i32);
pub const PROPERTY_DICTIONARY_ATTRIBUTES_COLLECTION: DICTIONARYPROPERTIES = DICTIONARYPROPERTIES(1024i32);
pub const PROPERTY_DICTIONARY_LOCATION: DICTIONARYPROPERTIES = DICTIONARYPROPERTIES(1025i32);
pub const PROPERTY_EVENTLOG_LOG_APPLICATION_EVENTS: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1026i32);
pub const PROPERTY_EVENTLOG_LOG_DEBUG: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1028i32);
pub const PROPERTY_EVENTLOG_LOG_MALFORMED: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1027i32);
pub const PROPERTY_IAS_AUDITORS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1028i32);
pub const PROPERTY_IAS_POLICIES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1025i32);
pub const PROPERTY_IAS_PROFILES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1026i32);
pub const PROPERTY_IAS_PROTOCOLS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1027i32);
pub const PROPERTY_IAS_PROXYPOLICIES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1030i32);
pub const PROPERTY_IAS_PROXYPROFILES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1031i32);
pub const PROPERTY_IAS_RADIUSSERVERGROUPS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1024i32);
pub const PROPERTY_IAS_REMEDIATIONSERVERGROUPS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1032i32);
pub const PROPERTY_IAS_REQUESTHANDLERS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1029i32);
pub const PROPERTY_IAS_SHVTEMPLATES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1033i32);
pub const PROPERTY_IPFILTER_ATTRIBUTES_COLLECTION: IPFILTERPROPERTIES = IPFILTERPROPERTIES(1024i32);
pub const PROPERTY_NAMES_REALMS: NAMESPROPERTIES = NAMESPROPERTIES(1026i32);
pub const PROPERTY_NAP_POLICIES_COLLECTION: NAPPROPERTIES = NAPPROPERTIES(1026i32);
pub const PROPERTY_NAS_VENDOR_ID: VENDORPROPERTIES = VENDORPROPERTIES(1024i32);
pub const PROPERTY_NTSAM_ALLOW_LM_AUTHENTICATION: NTSAMPROPERTIES = NTSAMPROPERTIES(1026i32);
pub const PROPERTY_POLICY_ACTION: POLICYPROPERTIES = POLICYPROPERTIES(1029i32);
pub const PROPERTY_POLICY_CONDITIONS_COLLECTION: POLICYPROPERTIES = POLICYPROPERTIES(1030i32);
pub const PROPERTY_POLICY_CONSTRAINT: POLICYPROPERTIES = POLICYPROPERTIES(1024i32);
pub const PROPERTY_POLICY_ENABLED: POLICYPROPERTIES = POLICYPROPERTIES(1031i32);
pub const PROPERTY_POLICY_MERIT: POLICYPROPERTIES = POLICYPROPERTIES(1025i32);
pub const PROPERTY_POLICY_PROFILE_NAME: POLICYPROPERTIES = POLICYPROPERTIES(1028i32);
pub const PROPERTY_POLICY_SOURCETAG: POLICYPROPERTIES = POLICYPROPERTIES(1032i32);
pub const PROPERTY_POLICY_UNUSED0: POLICYPROPERTIES = POLICYPROPERTIES(1026i32);
pub const PROPERTY_POLICY_UNUSED1: POLICYPROPERTIES = POLICYPROPERTIES(1027i32);
pub const PROPERTY_PROFILE_ATTRIBUTES_COLLECTION: PROFILEPROPERTIES = PROFILEPROPERTIES(1024i32);
pub const PROPERTY_PROFILE_IPFILTER_TEMPLATE_GUID: PROFILEPROPERTIES = PROFILEPROPERTIES(1025i32);
pub const PROPERTY_PROTOCOL_REQUEST_HANDLER: PROTOCOLPROPERTIES = PROTOCOLPROPERTIES(1026i32);
pub const PROPERTY_PROTOCOL_START: PROTOCOLPROPERTIES = PROTOCOLPROPERTIES(1027i32);
pub const PROPERTY_RADIUSPROXY_SERVERGROUPS: RADIUSPROXYPROPERTIES = RADIUSPROXYPROPERTIES(1026i32);
pub const PROPERTY_RADIUSSERVERGROUP_SERVERS_COLLECTION: RADIUSSERVERGROUPPROPERTIES = RADIUSSERVERGROUPPROPERTIES(1024i32);
pub const PROPERTY_RADIUSSERVER_ACCT_PORT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1026i32);
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1027i32);
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1037i32);
pub const PROPERTY_RADIUSSERVER_ADDRESS: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1028i32);
pub const PROPERTY_RADIUSSERVER_AUTH_PORT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1024i32);
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1025i32);
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1036i32);
pub const PROPERTY_RADIUSSERVER_BLACKOUT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1034i32);
pub const PROPERTY_RADIUSSERVER_FORWARD_ACCT_ONOFF: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1029i32);
pub const PROPERTY_RADIUSSERVER_MAX_LOST: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1033i32);
pub const PROPERTY_RADIUSSERVER_PRIORITY: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1030i32);
pub const PROPERTY_RADIUSSERVER_SEND_SIGNATURE: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1035i32);
pub const PROPERTY_RADIUSSERVER_TIMEOUT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1032i32);
pub const PROPERTY_RADIUSSERVER_WEIGHT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1031i32);
pub const PROPERTY_RADIUS_ACCOUNTING_PORT: RADIUSPROPERTIES = RADIUSPROPERTIES(1027i32);
pub const PROPERTY_RADIUS_AUTHENTICATION_PORT: RADIUSPROPERTIES = RADIUSPROPERTIES(1028i32);
pub const PROPERTY_RADIUS_CLIENTS_COLLECTION: RADIUSPROPERTIES = RADIUSPROPERTIES(1029i32);
pub const PROPERTY_RADIUS_VENDORS_COLLECTION: RADIUSPROPERTIES = RADIUSPROPERTIES(1030i32);
pub const PROPERTY_REMEDIATIONSERVERGROUP_SERVERS_COLLECTION: REMEDIATIONSERVERGROUPPROPERTIES = REMEDIATIONSERVERGROUPPROPERTIES(1024i32);
pub const PROPERTY_REMEDIATIONSERVERS_SERVERGROUPS: REMEDIATIONSERVERSPROPERTIES = REMEDIATIONSERVERSPROPERTIES(1026i32);
pub const PROPERTY_REMEDIATIONSERVER_ADDRESS: REMEDIATIONSERVERPROPERTIES = REMEDIATIONSERVERPROPERTIES(1024i32);
pub const PROPERTY_REMEDIATIONSERVER_FRIENDLY_NAME: REMEDIATIONSERVERPROPERTIES = REMEDIATIONSERVERPROPERTIES(1025i32);
pub const PROPERTY_SDO_CLASS: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(1i32);
pub const PROPERTY_SDO_DATASTORE_NAME: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(5i32);
pub const PROPERTY_SDO_DESCRIPTION: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(3i32);
pub const PROPERTY_SDO_ID: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(4i32);
pub const PROPERTY_SDO_NAME: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(2i32);
pub const PROPERTY_SDO_OPAQUE: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(7i32);
pub const PROPERTY_SDO_RESERVED: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(0i32);
pub const PROPERTY_SDO_START: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(1024i32);
pub const PROPERTY_SDO_TEMPLATE_GUID: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(6i32);
pub const PROPERTY_SHAREDSECRET_STRING: SHAREDSECRETPROPERTIES = SHAREDSECRETPROPERTIES(1024i32);
pub const PROPERTY_SHVCONFIG_LIST: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1026i32);
pub const PROPERTY_SHV_COMBINATION_TYPE: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1024i32);
pub const PROPERTY_SHV_LIST: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1025i32);
pub const PROPERTY_SHV_TEMPLATES_COLLECTION: NAPPROPERTIES = NAPPROPERTIES(1027i32);
pub const PROPERTY_TEMPLATES_CLIENTS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1032i32);
pub const PROPERTY_TEMPLATES_IPFILTERS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1035i32);
pub const PROPERTY_TEMPLATES_POLICIES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1024i32);
pub const PROPERTY_TEMPLATES_PROFILES_COLLECTION: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1026i32);
pub const PROPERTY_TEMPLATES_PROFILES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1025i32);
pub const PROPERTY_TEMPLATES_PROXYPOLICIES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1027i32);
pub const PROPERTY_TEMPLATES_PROXYPROFILES_COLLECTION: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1029i32);
pub const PROPERTY_TEMPLATES_PROXYPROFILES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1028i32);
pub const PROPERTY_TEMPLATES_RADIUSSERVERS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1033i32);
pub const PROPERTY_TEMPLATES_REMEDIATIONSERVERGROUPS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1030i32);
pub const PROPERTY_TEMPLATES_SHAREDSECRETS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1034i32);
pub const PROPERTY_TEMPLATES_SHVTEMPLATES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1031i32);
pub const PROPERTY_USER_ALLOW_DIALIN: USERPROPERTIES = USERPROPERTIES(1032i32);
pub const PROPERTY_USER_CALLING_STATION_ID: USERPROPERTIES = USERPROPERTIES(1024i32);
pub const PROPERTY_USER_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = USERPROPERTIES(1026i32);
pub const PROPERTY_USER_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = USERPROPERTIES(1036i32);
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = USERPROPERTIES(1038i32);
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = USERPROPERTIES(1034i32);
pub const PROPERTY_USER_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = USERPROPERTIES(1028i32);
pub const PROPERTY_USER_RADIUS_FRAMED_ROUTE: USERPROPERTIES = USERPROPERTIES(1027i32);
pub const PROPERTY_USER_SAVED_CALLING_STATION_ID: USERPROPERTIES = USERPROPERTIES(1025i32);
pub const PROPERTY_USER_SAVED_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = USERPROPERTIES(1029i32);
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = USERPROPERTIES(1037i32);
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = USERPROPERTIES(1039i32);
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = USERPROPERTIES(1035i32);
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = USERPROPERTIES(1031i32);
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_ROUTE: USERPROPERTIES = USERPROPERTIES(1030i32);
pub const PROPERTY_USER_SERVICE_TYPE: USERPROPERTIES = USERPROPERTIES(1033i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROTOCOLPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUSPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUSPROXYPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUSSERVERGROUPPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUSSERVERPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_ACTION(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RADIUS_ATTRIBUTE {
    pub dwAttrType: u32,
    pub fDataType: RADIUS_DATA_TYPE,
    pub cbDataLength: u32,
    pub Anonymous: RADIUS_ATTRIBUTE_0,
}
impl Default for RADIUS_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RADIUS_ATTRIBUTE_0 {
    pub dwValue: u32,
    pub lpValue: *const u8,
}
impl Default for RADIUS_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RADIUS_ATTRIBUTE_ACCT_AUTHENTIC: ATTRIBUTEID = ATTRIBUTEID(45u32);
pub const RADIUS_ATTRIBUTE_ACCT_DELAY_TIME: ATTRIBUTEID = ATTRIBUTEID(41u32);
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_OCTETS: ATTRIBUTEID = ATTRIBUTEID(42u32);
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_PACKETS: ATTRIBUTEID = ATTRIBUTEID(47u32);
pub const RADIUS_ATTRIBUTE_ACCT_INTERIM_INTERVAL: ATTRIBUTEID = ATTRIBUTEID(85u32);
pub const RADIUS_ATTRIBUTE_ACCT_LINK_COUNT: ATTRIBUTEID = ATTRIBUTEID(51u32);
pub const RADIUS_ATTRIBUTE_ACCT_MULTI_SSN_ID: ATTRIBUTEID = ATTRIBUTEID(50u32);
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_OCTETS: ATTRIBUTEID = ATTRIBUTEID(43u32);
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_PACKETS: ATTRIBUTEID = ATTRIBUTEID(48u32);
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_ID: ATTRIBUTEID = ATTRIBUTEID(44u32);
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_TIME: ATTRIBUTEID = ATTRIBUTEID(46u32);
pub const RADIUS_ATTRIBUTE_ACCT_STATUS_TYPE: ATTRIBUTEID = ATTRIBUTEID(40u32);
pub const RADIUS_ATTRIBUTE_ACCT_TERMINATE_CAUSE: ATTRIBUTEID = ATTRIBUTEID(49u32);
pub const RADIUS_ATTRIBUTE_ACCT_TUNNEL_CONN: ATTRIBUTEID = ATTRIBUTEID(68u32);
pub const RADIUS_ATTRIBUTE_ARAP_CHALLENGE_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(84u32);
pub const RADIUS_ATTRIBUTE_ARAP_FEATURES: ATTRIBUTEID = ATTRIBUTEID(71u32);
pub const RADIUS_ATTRIBUTE_ARAP_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(70u32);
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY: ATTRIBUTEID = ATTRIBUTEID(73u32);
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY_DATA: ATTRIBUTEID = ATTRIBUTEID(74u32);
pub const RADIUS_ATTRIBUTE_ARAP_ZONE_ACCESS: ATTRIBUTEID = ATTRIBUTEID(72u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RADIUS_ATTRIBUTE_ARRAY {
    pub cbSize: u32,
    pub Add: isize,
    pub AttributeAt: isize,
    pub GetSize: isize,
    pub InsertAt: isize,
    pub RemoveAt: isize,
    pub SetAt: isize,
}
pub const RADIUS_ATTRIBUTE_CALLBACK_ID: ATTRIBUTEID = ATTRIBUTEID(20u32);
pub const RADIUS_ATTRIBUTE_CALLBACK_NUMBER: ATTRIBUTEID = ATTRIBUTEID(19u32);
pub const RADIUS_ATTRIBUTE_CALLED_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(30u32);
pub const RADIUS_ATTRIBUTE_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(31u32);
pub const RADIUS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = ATTRIBUTEID(60u32);
pub const RADIUS_ATTRIBUTE_CHAP_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(3u32);
pub const RADIUS_ATTRIBUTE_CLASS: ATTRIBUTEID = ATTRIBUTEID(25u32);
pub const RADIUS_ATTRIBUTE_CONFIGURATION_TOKEN: ATTRIBUTEID = ATTRIBUTEID(78u32);
pub const RADIUS_ATTRIBUTE_CONNECT_INFO: ATTRIBUTEID = ATTRIBUTEID(77u32);
pub const RADIUS_ATTRIBUTE_EAP_MESSAGE: ATTRIBUTEID = ATTRIBUTEID(79u32);
pub const RADIUS_ATTRIBUTE_FILTER_ID: ATTRIBUTEID = ATTRIBUTEID(11u32);
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_LINK: ATTRIBUTEID = ATTRIBUTEID(37u32);
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_NET: ATTRIBUTEID = ATTRIBUTEID(38u32);
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_ZONE: ATTRIBUTEID = ATTRIBUTEID(39u32);
pub const RADIUS_ATTRIBUTE_FRAMED_COMPRESSION: ATTRIBUTEID = ATTRIBUTEID(13u32);
pub const RADIUS_ATTRIBUTE_FRAMED_INTERFACE_ID: ATTRIBUTEID = ATTRIBUTEID(96u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IPX_NETWORK: ATTRIBUTEID = ATTRIBUTEID(23u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IP_NETMASK: ATTRIBUTEID = ATTRIBUTEID(9u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_POOL: ATTRIBUTEID = ATTRIBUTEID(100u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_PREFIX: ATTRIBUTEID = ATTRIBUTEID(97u32);
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_ROUTE: ATTRIBUTEID = ATTRIBUTEID(99u32);
pub const RADIUS_ATTRIBUTE_FRAMED_MTU: ATTRIBUTEID = ATTRIBUTEID(12u32);
pub const RADIUS_ATTRIBUTE_FRAMED_PROTOCOL: ATTRIBUTEID = ATTRIBUTEID(7u32);
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTE: ATTRIBUTEID = ATTRIBUTEID(22u32);
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTING: ATTRIBUTEID = ATTRIBUTEID(10u32);
pub const RADIUS_ATTRIBUTE_IDLE_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(28u32);
pub const RADIUS_ATTRIBUTE_LOGIN_IP_HOST: ATTRIBUTEID = ATTRIBUTEID(14u32);
pub const RADIUS_ATTRIBUTE_LOGIN_IPv6_HOST: ATTRIBUTEID = ATTRIBUTEID(98u32);
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_GROUP: ATTRIBUTEID = ATTRIBUTEID(36u32);
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_NODE: ATTRIBUTEID = ATTRIBUTEID(35u32);
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_PORT: ATTRIBUTEID = ATTRIBUTEID(63u32);
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_SERVICE: ATTRIBUTEID = ATTRIBUTEID(34u32);
pub const RADIUS_ATTRIBUTE_LOGIN_SERVICE: ATTRIBUTEID = ATTRIBUTEID(15u32);
pub const RADIUS_ATTRIBUTE_LOGIN_TCP_PORT: ATTRIBUTEID = ATTRIBUTEID(16u32);
pub const RADIUS_ATTRIBUTE_NAS_IDENTIFIER: ATTRIBUTEID = ATTRIBUTEID(32u32);
pub const RADIUS_ATTRIBUTE_NAS_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4u32);
pub const RADIUS_ATTRIBUTE_NAS_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(95u32);
pub const RADIUS_ATTRIBUTE_NAS_PORT: ATTRIBUTEID = ATTRIBUTEID(5u32);
pub const RADIUS_ATTRIBUTE_NAS_PORT_TYPE: ATTRIBUTEID = ATTRIBUTEID(61u32);
pub const RADIUS_ATTRIBUTE_PASSWORD_RETRY: ATTRIBUTEID = ATTRIBUTEID(75u32);
pub const RADIUS_ATTRIBUTE_PORT_LIMIT: ATTRIBUTEID = ATTRIBUTEID(62u32);
pub const RADIUS_ATTRIBUTE_PROMPT: ATTRIBUTEID = ATTRIBUTEID(76u32);
pub const RADIUS_ATTRIBUTE_PROXY_STATE: ATTRIBUTEID = ATTRIBUTEID(33u32);
pub const RADIUS_ATTRIBUTE_REPLY_MESSAGE: ATTRIBUTEID = ATTRIBUTEID(18u32);
pub const RADIUS_ATTRIBUTE_SERVICE_TYPE: ATTRIBUTEID = ATTRIBUTEID(6u32);
pub const RADIUS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(27u32);
pub const RADIUS_ATTRIBUTE_SIGNATURE: ATTRIBUTEID = ATTRIBUTEID(80u32);
pub const RADIUS_ATTRIBUTE_STATE: ATTRIBUTEID = ATTRIBUTEID(24u32);
pub const RADIUS_ATTRIBUTE_TERMINATION_ACTION: ATTRIBUTEID = ATTRIBUTEID(29u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_ASSIGNMENT_ID: ATTRIBUTEID = ATTRIBUTEID(82u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_CLIENT_ENDPT: ATTRIBUTEID = ATTRIBUTEID(66u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_MEDIUM_TYPE: ATTRIBUTEID = ATTRIBUTEID(65u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(69u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_PREFERENCE: ATTRIBUTEID = ATTRIBUTEID(83u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_PVT_GROUP_ID: ATTRIBUTEID = ATTRIBUTEID(81u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_SERVER_ENDPT: ATTRIBUTEID = ATTRIBUTEID(67u32);
pub const RADIUS_ATTRIBUTE_TUNNEL_TYPE: ATTRIBUTEID = ATTRIBUTEID(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_ATTRIBUTE_TYPE(pub i32);
pub const RADIUS_ATTRIBUTE_UNASSIGNED1: ATTRIBUTEID = ATTRIBUTEID(17u32);
pub const RADIUS_ATTRIBUTE_UNASSIGNED2: ATTRIBUTEID = ATTRIBUTEID(21u32);
pub const RADIUS_ATTRIBUTE_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(1u32);
pub const RADIUS_ATTRIBUTE_USER_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(2u32);
pub const RADIUS_ATTRIBUTE_VENDOR_SPECIFIC: ATTRIBUTEID = ATTRIBUTEID(26u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_AUTHENTICATION_PROVIDER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_CODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_DATA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RADIUS_EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub repPoint: RADIUS_EXTENSION_POINT,
    pub rcRequestType: RADIUS_CODE,
    pub rcResponseType: RADIUS_CODE,
    pub GetRequest: isize,
    pub GetResponse: isize,
    pub SetResponseType: isize,
}
pub const RADIUS_EXTENSION_FREE_ATTRIBUTES: windows_core::PCSTR = windows_core::s!("RadiusExtensionFreeAttributes");
pub const RADIUS_EXTENSION_INIT: windows_core::PCSTR = windows_core::s!("RadiusExtensionInit");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_EXTENSION_POINT(pub i32);
pub const RADIUS_EXTENSION_PROCESS: windows_core::PCSTR = windows_core::s!("RadiusExtensionProcess");
pub const RADIUS_EXTENSION_PROCESS2: windows_core::PCSTR = windows_core::s!("RadiusExtensionProcess2");
pub const RADIUS_EXTENSION_PROCESS_EX: windows_core::PCSTR = windows_core::s!("RadiusExtensionProcessEx");
pub const RADIUS_EXTENSION_TERM: windows_core::PCSTR = windows_core::s!("RadiusExtensionTerm");
pub const RADIUS_EXTENSION_VERSION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RADIUS_REJECT_REASON_CODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RADIUS_VSA_FORMAT {
    pub VendorId: [u8; 4],
    pub VendorType: u8,
    pub VendorLength: u8,
    pub AttributeSpecific: [u8; 1],
}
impl Default for RADIUS_VSA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_LIMIT: ATTRIBUTEID = ATTRIBUTEID(4294967210u32);
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_TIME: ATTRIBUTEID = ATTRIBUTEID(4294967209u32);
pub const RAS_ATTRIBUTE_BAP_REQUIRED: ATTRIBUTEID = ATTRIBUTEID(4294967208u32);
pub const RAS_ATTRIBUTE_ENCRYPTION_POLICY: ATTRIBUTEID = ATTRIBUTEID(4294967207u32);
pub const RAS_ATTRIBUTE_ENCRYPTION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4294967206u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMEDIATIONSERVERGROUPPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMEDIATIONSERVERPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REMEDIATIONSERVERSPROPERTIES(pub i32);
pub const RESTRICTIONS: ATTRIBUTEINFO = ATTRIBUTEINFO(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVICE_TYPE(pub i32);
pub const SERVICE_TYPE_IAS: SERVICE_TYPE = SERVICE_TYPE(0i32);
pub const SERVICE_TYPE_MAX: SERVICE_TYPE = SERVICE_TYPE(3i32);
pub const SERVICE_TYPE_RAMGMTSVC: SERVICE_TYPE = SERVICE_TYPE(2i32);
pub const SERVICE_TYPE_RAS: SERVICE_TYPE = SERVICE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHAREDSECRETPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHVTEMPLATEPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHV_COMBINATION_TYPE(pub i32);
pub const SHV_COMBINATION_TYPE_ALL_FAIL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(1i32);
pub const SHV_COMBINATION_TYPE_ALL_PASS: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(0i32);
pub const SHV_COMBINATION_TYPE_MAX: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(7i32);
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_FAIL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(3i32);
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_INFECTED: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(4i32);
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_PASS: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(2i32);
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_TRANSITIONAL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(5i32);
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_UNKNOWN: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(6i32);
pub const SYNTAX: ATTRIBUTEINFO = ATTRIBUTEINFO(2i32);
pub const SYSTEM_TYPE_NT10_0_SERVER: IASOSTYPE = IASOSTYPE(13i32);
pub const SYSTEM_TYPE_NT10_0_WORKSTATION: IASOSTYPE = IASOSTYPE(6i32);
pub const SYSTEM_TYPE_NT4_SERVER: IASOSTYPE = IASOSTYPE(7i32);
pub const SYSTEM_TYPE_NT4_WORKSTATION: IASOSTYPE = IASOSTYPE(0i32);
pub const SYSTEM_TYPE_NT5_SERVER: IASOSTYPE = IASOSTYPE(8i32);
pub const SYSTEM_TYPE_NT5_WORKSTATION: IASOSTYPE = IASOSTYPE(1i32);
pub const SYSTEM_TYPE_NT6_1_SERVER: IASOSTYPE = IASOSTYPE(10i32);
pub const SYSTEM_TYPE_NT6_1_WORKSTATION: IASOSTYPE = IASOSTYPE(3i32);
pub const SYSTEM_TYPE_NT6_2_SERVER: IASOSTYPE = IASOSTYPE(11i32);
pub const SYSTEM_TYPE_NT6_2_WORKSTATION: IASOSTYPE = IASOSTYPE(4i32);
pub const SYSTEM_TYPE_NT6_3_SERVER: IASOSTYPE = IASOSTYPE(12i32);
pub const SYSTEM_TYPE_NT6_3_WORKSTATION: IASOSTYPE = IASOSTYPE(5i32);
pub const SYSTEM_TYPE_NT6_SERVER: IASOSTYPE = IASOSTYPE(9i32);
pub const SYSTEM_TYPE_NT6_WORKSTATION: IASOSTYPE = IASOSTYPE(2i32);
pub const SdoMachine: windows_core::GUID = windows_core::GUID::from_u128(0xe9218ae7_9e91_11d1_bf60_0080c7846bc0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TEMPLATESPROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USERPROPERTIES(pub i32);
pub const VENDORID: ATTRIBUTEINFO = ATTRIBUTEINFO(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VENDORPROPERTIES(pub i32);
pub const VENDORTYPE: ATTRIBUTEINFO = ATTRIBUTEINFO(7i32);
pub const raAccept: RADIUS_ACTION = RADIUS_ACTION(2i32);
pub const raContinue: RADIUS_ACTION = RADIUS_ACTION(0i32);
pub const raReject: RADIUS_ACTION = RADIUS_ACTION(1i32);
pub const rapMCIS: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(4i32);
pub const rapNone: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(6i32);
pub const rapODBC: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(5i32);
pub const rapProxy: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(2i32);
pub const rapUnknown: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(0i32);
pub const rapUsersFile: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(1i32);
pub const rapWindowsNT: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(3i32);
pub const ratAcctAuthentic: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(45i32);
pub const ratAcctDelayTime: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(41i32);
pub const ratAcctInputOctets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(42i32);
pub const ratAcctInputPackets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(47i32);
pub const ratAcctOutputOctets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(43i32);
pub const ratAcctOutputPackets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(48i32);
pub const ratAcctSessionId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(44i32);
pub const ratAcctSessionTime: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(46i32);
pub const ratAcctStatusType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(40i32);
pub const ratAcctTerminationCause: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(49i32);
pub const ratAuthenticator: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(264i32);
pub const ratCHAPChallenge: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(60i32);
pub const ratCHAPPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(3i32);
pub const ratCRPPolicyName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(275i32);
pub const ratCallbackId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(20i32);
pub const ratCallbackNumber: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(19i32);
pub const ratCalledStationId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(30i32);
pub const ratCallingStationId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(31i32);
pub const ratCertificateThumbprint: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(279i32);
pub const ratClass: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(25i32);
pub const ratClearTextPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(277i32);
pub const ratCode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(262i32);
pub const ratEAPTLV: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(273i32);
pub const ratExtensionState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(272i32);
pub const ratFQUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(269i32);
pub const ratFilterId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(11i32);
pub const ratFramedAppleTalkLink: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(37i32);
pub const ratFramedAppleTalkNetwork: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(38i32);
pub const ratFramedAppleTalkZone: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(39i32);
pub const ratFramedCompression: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(13i32);
pub const ratFramedIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(8i32);
pub const ratFramedIPNetmask: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(9i32);
pub const ratFramedIPXNetwork: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(23i32);
pub const ratFramedIPv6Pool: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(100i32);
pub const ratFramedIPv6Prefix: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(97i32);
pub const ratFramedIPv6Route: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(99i32);
pub const ratFramedInterfaceId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(96i32);
pub const ratFramedMTU: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(12i32);
pub const ratFramedProtocol: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(7i32);
pub const ratFramedRoute: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(22i32);
pub const ratFramedRouting: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(10i32);
pub const ratIdentifier: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(263i32);
pub const ratIdleTimeout: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(28i32);
pub const ratLoginIPHost: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(14i32);
pub const ratLoginIPv6Host: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(98i32);
pub const ratLoginLATGroup: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(36i32);
pub const ratLoginLATNode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(35i32);
pub const ratLoginLATService: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(34i32);
pub const ratLoginPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(16i32);
pub const ratLoginService: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(15i32);
pub const ratMediumType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(65i32);
pub const ratMinimum: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(0i32);
pub const ratNASIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(4i32);
pub const ratNASIPv6Address: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(95i32);
pub const ratNASIdentifier: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(32i32);
pub const ratNASPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(5i32);
pub const ratNASPortType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(61i32);
pub const ratPolicyName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(270i32);
pub const ratPortLimit: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(62i32);
pub const ratProvider: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(267i32);
pub const ratProviderName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(276i32);
pub const ratProxyState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(33i32);
pub const ratRejectReasonCode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(274i32);
pub const ratReplyMessage: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(18i32);
pub const ratServiceType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(6i32);
pub const ratSessionTimeout: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(27i32);
pub const ratSrcIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(265i32);
pub const ratSrcIPv6Address: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(278i32);
pub const ratSrcPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(266i32);
pub const ratState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(24i32);
pub const ratStrippedUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(268i32);
pub const ratTerminationAction: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(29i32);
pub const ratTunnelPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(69i32);
pub const ratTunnelPrivateGroupID: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(81i32);
pub const ratTunnelType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(64i32);
pub const ratUniqueId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(271i32);
pub const ratUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(1i32);
pub const ratUserPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(2i32);
pub const ratVendorSpecific: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(26i32);
pub const rcAccessAccept: RADIUS_CODE = RADIUS_CODE(2i32);
pub const rcAccessChallenge: RADIUS_CODE = RADIUS_CODE(11i32);
pub const rcAccessReject: RADIUS_CODE = RADIUS_CODE(3i32);
pub const rcAccessRequest: RADIUS_CODE = RADIUS_CODE(1i32);
pub const rcAccountingRequest: RADIUS_CODE = RADIUS_CODE(4i32);
pub const rcAccountingResponse: RADIUS_CODE = RADIUS_CODE(5i32);
pub const rcDiscard: RADIUS_CODE = RADIUS_CODE(256i32);
pub const rcUnknown: RADIUS_CODE = RADIUS_CODE(0i32);
pub const rdtAddress: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(2i32);
pub const rdtInteger: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(3i32);
pub const rdtIpv6Address: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(5i32);
pub const rdtString: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(1i32);
pub const rdtTime: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(4i32);
pub const rdtUnknown: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(0i32);
pub const repAuthentication: RADIUS_EXTENSION_POINT = RADIUS_EXTENSION_POINT(0i32);
pub const repAuthorization: RADIUS_EXTENSION_POINT = RADIUS_EXTENSION_POINT(1i32);
pub const rrrcAccountDisabled: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(2i32);
pub const rrrcAccountExpired: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(3i32);
pub const rrrcAccountUnknown: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(1i32);
pub const rrrcAuthenticationFailure: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(4i32);
pub const rrrcUndefined: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(0i32);
