use crate::co::WS;

const_type! { MB, u32,
	/// [`MessageBox`](crate::HWND::MessageBox) `uType` (`u32`).
	->
	ABORTRETRYIGNORE, 0x00000002
	CANCELTRYCONTINUE, 0x00000006
	HELP, 0x00004000
	OK, 0x00000000
	OKCANCEL, 0x00000001
	RETRYCANCEL, 0x00000005
	YESNO, 0x00000004
	YESNOCANCEL, 0x00000003

	ICONEXCLAMATION, 0x00000030
	ICONWARNING, Self::ICONEXCLAMATION.0
	ICONINFORMATION, 0x00000040
	ICONASTERISK, Self::ICONINFORMATION.0
	ICONQUESTION, 0x00000020
	ICONSTOP, Self::ICONERROR.0
	ICONERROR, 0x00000010
	ICONHAND, Self::ICONERROR.0

	DEFBUTTON1, 0x00000000
	DEFBUTTON2, 0x00000100
	DEFBUTTON3, 0x00000200
	DEFBUTTON4, 0x00000300

	APPLMODAL, 0x00000000
	SYSTEMMODAL, 0x00001000
	TASKMODAL, 0x00002000

	DEFAULT_DESKTOP_ONLY, 0x00020000
	RIGHT, 0x00080000
	RTLREADING, 0x00100000
	SETFOREGROUND, 0x00010000
	TOPMOST, 0x00040000
	SERVICE_NOTIFICATION, 0x00200000
}

const_type! { MCMV, u32,
	/// [`NMVIEWCHANGE`](crate::NMVIEWCHANGE) `dwOldView` and `dwNewView` (`u32`).
	->
	MONTH, 0
	YEAR, 1
	DECADE, 2
	CENTURY, 3
}

const_type_nm! { MCN,
	/// Month calendar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	->
	SELECT, Self::FIRST.0
	GETDAYSTATE, Self::FIRST.0 - 1
	SELCHANGE, Self::FIRST.0 - 3
	VIEWCHANGE, Self::FIRST.0 - 4
}
const_type_priv_values! { MCN,
	FIRST, -746
}

const_type! { MCSC, u8,
	/// [`DTM_GETMCCOLOR`](crate::msg::dtm::GetMcColor) color (`u8`).
	->
	BACKGROUND, 0
	TEXT, 1
	TITLEBK, 2
	TITLETEXT, 3
	MONTHBK, 4
	TRAILINGTEXT, 5
}

const_type_ws! { MCS,
	/// Month calendar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/month-calendar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	->
	DAYSTATE, 0x0001
	MULTISELECT, 0x0002
	WEEKNUMBERS, 0x0004
	NOTODAYCIRCLE, 0x0008
	NOTODAY, 0x0010
	NOTRAILINGDATES, 0x0040
	SHORTDAYSOFWEEK, 0x0080
	NOSELCHANGEONNAV, 0x0100
}

const_type! { MF, u32,
	/// [`AppendMenu`](crate::HMENU::AppendMenu) and
	/// [`InsertMenu`](crate::HMENU::InsertMenu) `uFlags` (`u32`).
	->
	INSERT, 0x00000000
	CHANGE, 0x00000080
	APPEND, 0x00000100
	DELETE, 0x00000200
	REMOVE, 0x00001000
	BYCOMMAND, 0x00000000
	BYPOSITION, 0x00000400
	SEPARATOR, 0x00000800
	ENABLED, 0x00000000
	GRAYED, 0x00000001
	DISABLED, 0x00000002
	UNCHECKED, 0x00000000
	CHECKED, 0x00000008
	USECHECKBITMAPS, 0x00000200
	STRING, 0x00000000
	BITMAP, 0x00000004
	OWNERDRAW, 0x00000100
	POPUP, 0x00000010
	MENUBARBREAK, 0x00000020
	MENUBREAK, 0x00000040
	UNHILITE, 0x00000000
	HILITE, 0x00000080
	DEFAULT, 0x00001000
	SYSMENU, 0x00002000
	HELP, 0x00004000
	RIGHTJUSTIFY, 0x00004000
	MOUSESELECT, 0x00008000
}

const_type! { MFS, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fState` (`u32`).
	->
	GRAYED, 0x00000003
	DISABLED, MFS::GRAYED.0
	CHECKED, MF::CHECKED.0
	HILITE, MF::HILITE.0
	ENABLED, MF::ENABLED.0
	UNCHECKED, MF::UNCHECKED.0
	UNHILITE, MF::UNHILITE.0
	DEFAULT, MF::DEFAULT.0
}

const_type! { MFT, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fType` (`u32`).
	->
	STRING, MF::STRING.0
	BITMAP, MF::BITMAP.0
	MENUBARBREAK, MF::MENUBARBREAK.0
	MENUBREAK, MF::MENUBREAK.0
	OWNERDRAW, MF::OWNERDRAW.0
	RADIOCHECK, 0x00000200
	SEPARATOR, MF::SEPARATOR.0
	RIGHTORDER, 0x00002000
	RIGHTJUSTIFY, MF::RIGHTJUSTIFY.0
}

const_type! { MIM, u32,
	/// [`MENUINFO`](crate::MENUINFO) `fMask` (`u32`).
	->
	MAXHEIGHT, 0x00000001
	BACKGROUND, 0x00000002
	HELPID, 0x00000004
	MENUDATA, 0x00000008
	STYLE, 0x00000010
	APPLYTOSUBMENUS, 0x80000000
}

const_type! { MIIM, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fMask` (`u32`).
	->
	MAXHEIGHT, 0x00000001
	BACKGROUND, 0x00000002
	HELPID, 0x00000004
	MENUDATA, 0x00000008
	STYLE, 0x00000010
	APPLYTOSUBMENUS, 0x80000000
}

const_type! { MK, u16,
	/// [`WM_LBUTTONDOWN`](crate::msg::wm::LButtonDown) (and similar) virtual
	/// keys (`u16`).
	->
	LBUTTON, 0x0001
	RBUTTON, 0x0002
	SHIFT, 0x0004
	CONTROL, 0x0008
	MBUTTON, 0x0010
	XBUTTON1, 0x0020
	XBUTTON2, 0x0040
}

const_type! { MNS, u32,
	/// [`MENUINFO`](crate::MENUINFO) `dwStyle` (`u32`).
	->
	NOCHECK, 0x80000000
	MODELESS, 0x40000000
	DRAGDROP, 0x20000000
	AUTODISMISS, 0x10000000
	NOTIFYBYPOS, 0x08000000
	CHECKORBMP, 0x04000000
}

const_type! { MSGF, u8,
	/// [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) reason (`u8`).
	->
	DIALOGBOX, 0
	MENU, 2
}

const_type! { NM, i32,
	/// [`WM_NOTIFY`](crate::msg::wm::Notify) notification codes (`i32`).
	///
	/// Control-specific notification codes have their own types, which are
	/// convertible to `NM`.
	->
	OUTOFMEMORY, Self::FIRST.0 - 1
	CLICK, Self::FIRST.0 - 2
	DBLCLK, Self::FIRST.0 - 3
	RETURN, Self::FIRST.0 - 4
	RCLICK, Self::FIRST.0 - 5
	RDBLCLK, Self::FIRST.0 - 6
	SETFOCUS, Self::FIRST.0 - 7
	KILLFOCUS, Self::FIRST.0 - 8
	CUSTOMDRAW, Self::FIRST.0 - 12
	HOVER, Self::FIRST.0 - 13
	NCHITTEST, Self::FIRST.0 - 14
	KEYDOWN, Self::FIRST.0 - 15
	RELEASEDCAPTURE, Self::FIRST.0 - 16
	SETCURSOR, Self::FIRST.0 - 17
	CHAR, Self::FIRST.0 - 18
	TOOLTIPSCREATED, Self::FIRST.0 - 19
	LDOWN, Self::FIRST.0 - 20
	RDOWN, Self::FIRST.0 - 21
	THEMECHANGED, Self::FIRST.0 - 22
}
const_type_priv_values! { NM,
	FIRST, 0
}
