use super::common::{CommonPluginInfo, PluginHandle};
use std::ffi::{c_char, c_void, CString};

#[repr(C)]
#[allow(non_snake_case)]
pub struct SKSEInterface {
    pub skseVersion: u32,
    pub runtimeVersion: u32,
    pub editorVersion: u32,
    pub isEditor: u32,

    pub QueryInterface: extern "C" fn(u32) -> *const c_void,
    pub GetPluginHandle: extern "C" fn() -> PluginHandle,
    pub GetReleaseIndex: extern "C" fn() -> u32,
    pub GetPluginInfo: extern "C" fn(name: *const c_char) -> *const CommonPluginInfo,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SKSEPluginVersionData {
    pub dataVersion: u32,
    pub pluginVersion: u32,
    pub name: [u8; 256],
    pub author: [u8; 256],
    pub supportEmail: [u8; 252],
    pub versionIndependenceEx: u32,
    pub versionIndependence: u32,
    pub compatibleVersions: [u32; 16],
    pub seVersionRequired: u32,
}

#[allow(dead_code)]
impl SKSEPluginVersionData {
    pub const VERSION: u32 = 1;

    pub const VERSION_INDEPENDENT_ADDRESS_LIBRARY_POST_AE: u32 = 1 << 0;
    pub const VERSION_INDEPENDENT_SIGNATURES: u32 = 1 << 1;
    pub const VERSION_INDEPENDENT_STRUCTS_POST_629: u32 = 1 << 2;

    pub const VERSION_INDEPENDENT_EX_NO_STRUCT_USE: u32 = 1 << 0;
}

#[no_mangle]
pub static SKSE_PLUGIN_VERSION: SKSEPluginVersionData = SKSEPluginVersionData {
    dataVersion: 1,
    pluginVersion: 3,
    name: *concat_bytes!(b"Skyrim Priority", [0u8; 241]),
    author: *concat_bytes!(b"Boring3", [0u8; 249]),
    supportEmail: [0; 252],
    versionIndependenceEx: SKSEPluginVersionData::VERSION_INDEPENDENT_EX_NO_STRUCT_USE,
    versionIndependence: SKSEPluginVersionData::VERSION_INDEPENDENT_SIGNATURES,
    compatibleVersions: [0; 16],
    seVersionRequired: 0,
};

#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Query(
    interface: *const SKSEInterface,
    info: *mut CommonPluginInfo,
) -> bool {
    (*info).infoVersion = 1;
    (*info).name = CString::new("Skyrim Priority")
        .expect("could not create CString")
        .into_raw();
    (*info).version = 1;

    return (*interface).isEditor == 0;
}

#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Load(_interface: *const SKSEInterface) -> bool {
    true
}
