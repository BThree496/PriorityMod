use super::common::{CommonPluginInfo, PluginHandle};
use std::ffi::{c_void, CString};

#[repr(C)]
#[allow(non_snake_case)]
pub struct F4SEInterface {
    pub f4seVersion: u32,
    pub runtimeVersion: u32,
    pub editorVersion: u32,
    pub isEditor: u32,

    pub QueryInterface: extern "C" fn(u32) -> *const c_void,
    pub GetPluginHandle: extern "C" fn() -> PluginHandle,
    pub GetReleaseIndex: extern "C" fn() -> u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct F4SEPluginVersionData {
    pub dataVersion: u32,
    pub pluginVersion: u32,
    pub name: [u8; 256],
    pub author: [u8; 256],
    pub addressIndependence: u32,
    pub structureIndependence: u32,
    pub compatibleVersions: [u32; 16],
    pub seVersionRequired: u32,
    pub reservedNonBreaking: u32, // bitfield. set to 0
    pub reservedBreaking: u32,    // bitfield. set to 0
    pub reserved: [u8; 512],      // set to 0
}

impl F4SEPluginVersionData {
    pub const VERSION: u32 = 1;

    pub const ADDRESS_INDEPENDENCE_SIGNATURES: u32 = 1 << 0;
    pub const ADDRESS_INDEPENDENCE_ADDRESS_LIBRARY_1_10_980: u32 = 1 << 1;

    pub const STRUCTURE_INDEPENDENCE_NO_STRUCTS: u32 = 1 << 0;
    pub const STRUCTURE_INDEPENDENCE_1_10_980_LAYOUT: u32 = 1 << 0;
}

#[no_mangle]
pub static F4SEPlugin_Version: F4SEPluginVersionData = F4SEPluginVersionData {
    dataVersion: F4SEPluginVersionData::VERSION,
    pluginVersion: 3,
    name: *concat_bytes!(b"Fallout Priority", [0u8; 240]),
    author: *concat_bytes!(b"Boring3", [0u8; 249]),
    addressIndependence: F4SEPluginVersionData::ADDRESS_INDEPENDENCE_SIGNATURES,
    structureIndependence: F4SEPluginVersionData::STRUCTURE_INDEPENDENCE_NO_STRUCTS,
    compatibleVersions: [0; 16],
    seVersionRequired: 0,

    reservedNonBreaking: 0, // bitfield. set to 0
    reservedBreaking: 0,    // bitfield. set to 0
    reserved: [0u8; 512],   // set to 0
};

#[no_mangle]
pub unsafe extern "C" fn F4SEPlugin_Query(
    interface: *const F4SEInterface,
    info: *mut CommonPluginInfo,
) -> bool {
    (*info).infoVersion = 1;
    (*info).name = CString::new("Fallout Priority")
        .expect("could not create CString")
        .into_raw();
    (*info).version = 1;

    return (*interface).isEditor == 0;
}

#[no_mangle]
pub unsafe extern "C" fn F4SEPlugin_Load(_interface: *const F4SEInterface) -> bool {
    true
}
