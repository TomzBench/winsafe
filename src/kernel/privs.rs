#![allow(dead_code, non_snake_case)]

use crate::co::ERROR;
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::ffi_types::{BOOL, HANDLE};

pub(crate) const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const INFINITE: u32 = 0xffff_ffff;
pub(crate) const INVALID_FILE_ATTRIBUTES: i32 = -1;
pub(crate) const LMEM_INVALID_HANDLE: u32 = 0x8000;
pub(crate) const MAX_COMPUTERNAME_LENGTH: usize = 15;
pub(crate) const MAX_MODULE_NAME32: usize = 255;
pub(crate) const MAX_PATH: usize = 260;
pub(crate) const SECURITY_DESCRIPTOR_REVISION: u32 = 1;

/// [`IS_INTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-is_intresource)
/// macro.
pub(crate) const fn IS_INTRESOURCE(val: *const u16) -> bool {
	(unsafe { std::mem::transmute::<_, usize>(val) } >> 16) == 0
}

/// [`MAKEINTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
/// macro.
pub(crate) const fn MAKEINTRESOURCE(val: isize) -> *const u16 {
	val as u16 as _
}

/// If value is `FALSE`, yields `Err(GetLastError)`, otherwise `Ok()`.
pub(crate) fn bool_to_sysresult(expr: BOOL) -> SysResult<()> {
	match expr {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// If pointer is null, yields `Err(GetLastError)`, otherwise `Ok(op(ptr))`.
pub(crate) fn ptr_to_sysresult<U, F>(ptr: HANDLE, op: F) -> SysResult<U>
	where F: FnOnce(HANDLE) -> U,
{
	if ptr.is_null() {
		Err(GetLastError())
	} else {
		Ok(op(ptr))
	}
}

/// If value is `ERROR::SUCCESS`, yields `Ok(())`, otherwise `Err(err)`.
pub(crate) const fn error_to_sysresult(lstatus: i32) -> SysResult<()> {
	match ERROR(lstatus as _) {
		ERROR::SUCCESS => Ok(()),
		err => Err(err),
	}
}

/// Converts a string to an ISO-8859-1 null-terminated byte array.
pub(crate) fn str_to_iso88591(s: &str) -> Vec<u8> {
	s.chars().map(|ch| ch as u8)
		.chain(std::iter::once(0)) // append a terminating null
		.collect()
}

/// Parses a null-delimited multi-string, which must terminate with two nulls.
pub(crate) fn parse_multi_z_str(src: *const u16) -> Vec<String> {
	let mut src = src;
	let mut strings = Vec::<String>::default();
	let mut i = 0;

	loop {
		if unsafe { *src.add(i) } == 0 {
			let slice = unsafe { std::slice::from_raw_parts(src, i) };
			if slice.is_empty() {
				break;
			}
			strings.push(WString::from_wchars_slice(slice).to_string());
			src = unsafe { src.add(i + 1) };
			i = 0;
		} else {
			i += 1;
		}
	}
	strings
}
