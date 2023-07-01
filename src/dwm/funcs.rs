#![allow(non_snake_case)]

use crate::dwm;
use crate::kernel::ffi_types::BOOL;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;

/// [`DwmEnableMMCSS`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmenablemmcss)
/// function.
pub fn DwmEnableMMCSS(enable: bool) -> HrResult<()> {
	ok_to_hrresult(unsafe { dwm::ffi::DwmEnableMMCSS(enable as _) })
}

/// [`DwmFlush`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmflush)
/// function.
pub fn DwmFlush() -> HrResult<()> {
	ok_to_hrresult(unsafe { dwm::ffi::DwmFlush() })
}

/// [`DwmGetColorizationColor`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetcolorizationcolor)
/// function.
///
/// Returns the color in the `0xAARRGGBB` format, and whether the color is an
/// opaque blend.
#[must_use]
pub fn DwmGetColorizationColor() -> HrResult<(u32, bool)> {
	let mut colorization = u32::default();
	let mut opaque_blend: BOOL = 0;

	ok_to_hrresult(
		unsafe {
			dwm::ffi::DwmGetColorizationColor(&mut colorization, &mut opaque_blend)
		},
	).map(|_| (colorization, opaque_blend != 0))
}

/// [`DwmIsCompositionEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmiscompositionenabled)
/// function.
#[must_use]
pub fn DwmIsCompositionEnabled() -> HrResult<bool> {
	let mut pf_enabled: BOOL = 0;
	ok_to_hrresult(unsafe { dwm::ffi::DwmIsCompositionEnabled(&mut pf_enabled) })
		.map(|_| pf_enabled != 0)
}
