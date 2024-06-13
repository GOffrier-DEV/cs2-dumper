use std::ffi::c_char;

use memflow::prelude::v1::*;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaMetadataEntryData {
    pub name: Pointer64<ReprCString>,                 // 0x0000
    pub network_value: Pointer64<SchemaNetworkValue>, // 0x0008
}

#[repr(C)]
pub struct SchemaNetworkValue {
    pub value: SchemaNetworkValueUnion, // 0x0000
}

unsafe impl Pod for SchemaNetworkValue {}

#[repr(C)]
pub union SchemaNetworkValueUnion {
    pub name_ptr: Pointer64<ReprCString>,
    pub int_value: i32,
    pub float_value: f32,
    pub ptr: Pointer64<()>,
    pub var_value: SchemaVarName,
    pub name_value: [c_char; 32],
}

#[derive(Clone, Copy, Pod)]
#[repr(C)]
pub struct SchemaVarName {
    pub name: Pointer64<ReprCString>,      // 0x0000
    pub type_name: Pointer64<ReprCString>, // 0x0008
}
