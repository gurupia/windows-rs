#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WAIT_EVENT(pub u32);
impl windows_core::TypeKind for WAIT_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WAIT_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WAIT_EVENT").field(&self.0).finish()
    }
}
pub const WAIT_IO_COMPLETION: WAIT_EVENT = WAIT_EVENT(192u32);
pub const WAIT_TIMEOUT: WAIT_EVENT = WAIT_EVENT(258u32);
