use std::cell::Cell;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HtreeitemTvi;
use crate::handles::{HTREEITEM, HWND};
use crate::msg::tvm;
use crate::privs::MAX_PATH;
use crate::structs::{TVINSERTSTRUCT, TVITEMEX};
use crate::WString;

/// Exposes item methods of a [`TreeView`](crate::gui::TreeView) control.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct TreeViewItems {
	hwnd_ptr: Cell<NonNull<HWND>>,
}

impl TreeViewItems {
	pub(crate) fn new(hwnd_ref: &HWND) -> TreeViewItems {
		Self {
			hwnd_ptr: Cell::new(NonNull::from(hwnd_ref)), // ref implicitly converted to pointer
		}
	}

	pub(crate) fn set_hwnd_ref(&self, hwnd_ref: &HWND) {
		self.hwnd_ptr.replace(NonNull::from(hwnd_ref)); // ref implicitly converted to pointer
	}

	pub(crate) fn hwnd(&self) -> HWND {
		unsafe { *self.hwnd_ptr.get().as_ref() }
	}

	/// Adds a new child item by sending a
	/// [`TVN_INSERTITEM`](crate::msg::tvm::InsertItem) message.
	pub fn add_child(&self,
		hparent: HTREEITEM,
		text: &str,
		icon_index: Option<u32>) -> WinResult<HTREEITEM>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}
		tvix.set_pszText(&mut buf);

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.hParent = hparent;
		tvis.set_hInsertAfter(HtreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		self.hwnd().SendMessage(tvm::InsertItem { tvinsertstruct: &mut tvis })
	}

	/// Adds a new root item by sending a
	/// [`TVN_INSERTITEM`](crate::msg::tvm::InsertItem) message.
	pub fn add_root(&self,
		text: &str, icon_index: Option<u32>) -> WinResult<HTREEITEM>
	{
		let mut buf = WString::from_str(text);

		let mut tvix = TVITEMEX::default();
		tvix.mask = co::TVIF::TEXT;
		if let Some(icon_index) = icon_index {
			tvix.mask |= co::TVIF::IMAGE;
			tvix.iImage = icon_index as _;
		}
		tvix.set_pszText(&mut buf);

		let mut tvis = TVINSERTSTRUCT::default();
		tvis.set_hInsertAfter(HtreeitemTvi::Tvi(co::TVI::LAST));
		tvis.itemex = tvix;

		self.hwnd().SendMessage(tvm::InsertItem { tvinsertstruct: &mut tvis })
	}

	/// Retrieves the children of the given item by sending
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) messages.
	///
	/// To retrieve the root nodes, pass `None` as `hparent`.
	pub fn children(&self, hparent: Option<HTREEITEM>) -> Vec<HTREEITEM> {
		let mut hchildren = Vec::default();

		let mut hfound = self.hwnd().SendMessage(tvm::GetNextItem {
			which: co::TVGN::CHILD,
			hitem: hparent.unwrap_or(HTREEITEM::NULL), // search first children
		});

		while let Some(hitem_found) = hfound {
			hchildren.push(hitem_found);
			hfound = self.next_sibling(hitem_found); // then search siblings
		}

		hchildren
	}

	/// Deletes an item by sending a
	/// [`TVN_DELETEITEM`](crate::msg::tvm::DeleteItem) message.
	pub fn delete(&self, hitem: HTREEITEM) -> WinResult<()> {
		self.hwnd().SendMessage(tvm::DeleteItem { hitem })
	}

	/// Deletes all items by sending a
	/// [`TVN_DELETEITEM`](crate::msg::tvm::DeleteItem) message.
	pub fn delete_all(&self) -> WinResult<()> {
		self.hwnd().SendMessage(tvm::DeleteItem { hitem: HTREEITEM::NULL })
	}

	/// Retrieves the total number of items by sending a
	/// [`TVN_GETCOUNT`](crate::msg::tvm::GetCount) message.
	pub fn count(&self) -> u32 {
		self.hwnd().SendMessage(tvm::GetCount {})
	}

	/// Expands or collapse an item by sending a
	/// [`TVM_EXPAND`](crate::msg::tvm::Expand) message.
	pub fn expand(&self, hitem: HTREEITEM, expand: bool) -> WinResult<()> {
		self.hwnd().SendMessage(tvm::Expand {
			hitem,
			action: if expand { co::TVE::EXPAND } else { co::TVE::COLLAPSE },
		})
	}

	/// Tells if the item is a root by sending a
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) message.
	pub fn is_root(&self, hitem: HTREEITEM) -> bool {
		self.parent(hitem).is_none()
	}

	/// Retrieves the next sibling of the item by sending a
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) message.
	pub fn next_sibling(&self, hitem: HTREEITEM) -> Option<HTREEITEM> {
		self.hwnd().SendMessage(tvm::GetNextItem {
			which: co::TVGN::NEXT,
			hitem,
		})
	}

	/// Retrieves the parent of the item by sending a
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) message.
	pub fn parent(&self, hitem: HTREEITEM) -> Option<HTREEITEM> {
		self.hwnd().SendMessage(tvm::GetNextItem {
			which: co::TVGN::PARENT,
			hitem,
		})
	}

	/// Retrieves the previous sibling of the item by sending a
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) message.
	pub fn prev_sibling(&self, hitem: HTREEITEM) -> Option<HTREEITEM> {
		self.hwnd().SendMessage(tvm::GetNextItem {
			which: co::TVGN::PREVIOUS,
			hitem,
		})
	}

	/// Retrieves the selected item by sending a
	/// [`TVN_GETNEXTITEM`](crate::msg::tvm::GetNextItem) message.
	pub fn selected(&self, hitem: HTREEITEM) -> Option<HTREEITEM> {
		self.hwnd().SendMessage(tvm::GetNextItem {
			which: co::TVGN::CARET,
			hitem,
		})
	}

	/// Sets the text of the item by sending a
	/// [`TVN_SETITEM`](crate::msg::tvm::SetItem) message.
	pub fn set_text(&self, hitem: HTREEITEM, text: &str) -> WinResult<()> {
		let mut buf = WString::from_str(text);

		let mut tvi = TVITEMEX::default();
		tvi.hItem = hitem;
		tvi.mask = co::TVIF::TEXT;
		tvi.set_pszText(&mut buf);

		self.hwnd().SendMessage(tvm::SetItem { tvitem: &tvi })
	}

	/// Retrieves the text of the item by sending a
	/// [`TVN_GETITEM`](crate::msg::tvm::GetItem) message.
	///
	/// The passed buffer will be automatically allocated.
	///
	/// This method can be more performant than
	/// [`text_str`](crate::gui::TreeViewItems::text_str) because the buffer can be
	/// reused, avoiding multiple allocations. However, it has the inconvenient
	/// of the manual conversion from [`WString`](crate::WString) to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, HTREEITEM, WString};
	///
	/// let my_tree: gui::ListView; // initialized somewhere
	/// let my_item: HTREEITEM;
	///
	/// let mut buf = WString::default();
	/// my_tree.items().text(my_item, &mut buf).unwrap();
	///
	/// println!("Text: {}", buf.to_string());
	/// ```
	pub fn text(&self,
		hitem: HTREEITEM, mut buf: &mut WString) -> WinResult<()>
	{
		buf.realloc_buffer(MAX_PATH + 1); // arbitrary

		let mut tvi = TVITEMEX::default();
		tvi.hItem = hitem;
		tvi.mask = co::TVIF::TEXT;
		tvi.set_pszText(&mut buf);

		self.hwnd().SendMessage(tvm::GetItem { tvitem: &mut tvi })
	}

	/// A more convenient [`text`](crate::gui::TreeViewItems::text), which
	/// directly returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, HTREEITEM};
	///
	/// let my_tree: gui::TreeView; // initialized somewhere
	/// let my_item: HTREEITEM;
	///
	/// println!("Text: {}", my_tree.items().text(my_item).unwrap());
	/// ```
	pub fn text_str(&self, hitem: HTREEITEM) -> WinResult<String> {
		let mut buf = WString::default();
		self.text(hitem, &mut buf)?;
		Ok(buf.to_string())
	}
}
