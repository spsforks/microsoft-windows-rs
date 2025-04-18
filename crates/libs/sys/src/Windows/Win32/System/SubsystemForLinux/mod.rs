windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslConfigureDistribution(distributionname : windows_sys::core::PCWSTR, defaultuid : u32, wsldistributionflags : WSL_DISTRIBUTION_FLAGS) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslGetDistributionConfiguration(distributionname : windows_sys::core::PCWSTR, distributionversion : *mut u32, defaultuid : *mut u32, wsldistributionflags : *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables : *mut *mut windows_sys::core::PSTR, defaultenvironmentvariablecount : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslIsDistributionRegistered(distributionname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslLaunch(distributionname : windows_sys::core::PCWSTR, command : windows_sys::core::PCWSTR, usecurrentworkingdirectory : windows_sys::core::BOOL, stdin : super::super::Foundation:: HANDLE, stdout : super::super::Foundation:: HANDLE, stderr : super::super::Foundation:: HANDLE, process : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslLaunchInteractive(distributionname : windows_sys::core::PCWSTR, command : windows_sys::core::PCWSTR, usecurrentworkingdirectory : windows_sys::core::BOOL, exitcode : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslRegisterDistribution(distributionname : windows_sys::core::PCWSTR, targzfilename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("api-ms-win-wsl-api-l1-1-0.dll" "system" fn WslUnregisterDistribution(distributionname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub type WSL_DISTRIBUTION_FLAGS = i32;
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = 2i32;
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = 4i32;
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = 1i32;
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = 0i32;
