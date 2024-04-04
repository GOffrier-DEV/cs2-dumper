use memflow::prelude::v1::*;

#[repr(C)]
pub struct KeyboardKey {
    pad_0000: [u8; 0x8],              // 0x0000
    pub name: Pointer64<ReprCString>, // 0x0008
    pad_0010: [u8; 0x20],             // 0x0010
    pub state: u32,                   // 0x0030
    pad_0034: [u8; 0x54],             // 0x0034
    pub next: Pointer64<KeyboardKey>, // 0x0088
}

unsafe impl Pod for KeyboardKey {}
