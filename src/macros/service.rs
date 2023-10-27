//! This macros file essentially exists to export the
//! [`winsafe::start_service_ctrl_dispatcher!`] macro
#[macro_export]
macro_rules! __replace_expr {
    ($_t:literal $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! __count {
    ($($literals:literal)*) => {0usize $(+ $crate::__replace_expr!($literals 1usize))*};
}

#[macro_export]
macro_rules! __count_null_terminated {
    ($($literals:literal)*) => {1usize $(+ $crate::__replace_expr!($literals 1usize))*};
}

#[macro_export]
macro_rules! __call_with_service_table_entry {
    ($func:expr, [ $( ($name:literal, $service:expr) ),* ]) => {{
        const __WINSAFE_SERVICES_SZ: usize = $crate::__count!($($name)*);
        const __WINSAFE_TABLE_SZ: usize = $crate::__count_null_terminated!($($name)*);
        let mut __winsafe_service_names: [$crate::WString; __WINSAFE_SERVICES_SZ] = [$($crate::WString::from_str($name)),*];
        let __winsafe_service_procs: [$crate::SERVICEMAINFUNCTION; __WINSAFE_SERVICES_SZ] = [$($service),*];
        let mut table = std::mem::MaybeUninit::<[$crate::SERVICE_TABLE_ENTRY; __WINSAFE_TABLE_SZ]>::uninit();
        let ptr: &mut [$crate::SERVICE_TABLE_ENTRY; __WINSAFE_TABLE_SZ] = unsafe { &mut *table.as_mut_ptr() as _ };
        for idx in 0 .. __WINSAFE_SERVICES_SZ {
            ptr[idx].lpServiceName = unsafe { __winsafe_service_names[idx].as_mut_ptr() };
            ptr[idx].lpServiceProc = Some(__winsafe_service_procs[idx]);
        }
        ptr[__WINSAFE_TABLE_SZ - 1].lpServiceName = std::ptr::null_mut();
        ptr[__WINSAFE_TABLE_SZ - 1].lpServiceProc = None;
        let table = unsafe { table.assume_init() };
        ($func)(&table)
    }};
}

#[macro_export]
macro_rules! start_service_ctrl_dispatcher {
    [ $( ($name:literal, $service:expr) ),* ] => {{
        const __WINSAFE_SERVICES_SZ: usize = $crate::__count!($($name)*);
        const __WINSAFE_TABLE_SZ: usize = $crate::__count_null_terminated!($($name)*);
        let mut __winsafe_service_names: [$crate::WString; __WINSAFE_SERVICES_SZ] = [$($crate::WString::from_str($name)),*];
        let __winsafe_service_procs: [$crate::SERVICEMAINFUNCTION; __WINSAFE_SERVICES_SZ] = [$($service),*];
        let mut table = std::mem::MaybeUninit::<[$crate::SERVICE_TABLE_ENTRY; __WINSAFE_TABLE_SZ]>::uninit();
        let ptr: &mut [$crate::SERVICE_TABLE_ENTRY; __WINSAFE_TABLE_SZ] = unsafe { &mut *table.as_mut_ptr() as _ };
        for idx in 0 .. __WINSAFE_SERVICES_SZ {
            ptr[idx].lpServiceName = unsafe { __winsafe_service_names[idx].as_mut_ptr() };
            ptr[idx].lpServiceProc = Some(__winsafe_service_procs[idx]);
        }
        ptr[__WINSAFE_TABLE_SZ - 1].lpServiceName = std::ptr::null_mut();
        ptr[__WINSAFE_TABLE_SZ - 1].lpServiceProc = None;
        let table = unsafe { table.assume_init() };
        StartServiceCtrlDispatcher(&table)
    }};
}

mod test {
    extern "system" fn _my_service_a(_argc: u32, _argv: *mut *mut u16) {}
    extern "system" fn _my_service_b(_argc: u32, _argv: *mut *mut u16) {}
    extern "system" fn _my_service_c(_argc: u32, _argv: *mut *mut u16) {}
    #[test]
    fn test_file_names() {
        fn test_fn(table: &[crate::SERVICE_TABLE_ENTRY; 4]) -> crate::SysResult<()> {
            use crate::WString;
            unsafe {
                assert_eq!(
                    WString::from_wchars_nullt(table[0].lpServiceName),
                    WString::from_str("MyServiceA")
                );
                assert_eq!(
                    WString::from_wchars_nullt(table[1].lpServiceName),
                    WString::from_str("MyServiceB")
                );
                assert_eq!(
                    WString::from_wchars_nullt(table[2].lpServiceName),
                    WString::from_str("MyServiceC")
                );
                assert_eq!(table[3].lpServiceName, std::ptr::null_mut());
                assert_eq!(table[3].lpServiceProc, None);
                Ok(())
            }
        }
        __call_with_service_table_entry!(
            test_fn,
            [
                ("MyServiceA", _my_service_a),
                ("MyServiceB", _my_service_b),
                ("MyServiceC", _my_service_c)
            ]
        )
        .unwrap();
    }
}
