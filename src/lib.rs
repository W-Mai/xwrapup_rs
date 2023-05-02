#![no_std]

#[allow(dead_code)]
extern "C" {
    pub fn clock_ms() -> i32;
    pub fn log_print(ptr: *const u8, len: usize);
    pub fn _debug(ptr: *const u8, len: usize) -> i32;
    pub fn log_print_i32(num: i32);
}

#[allow(dead_code)]
pub mod xwu {
    use core::ptr::null_mut;


    #[link(wasm_import_module = "xwu")]
    extern "C" {
        #[link_name = "wu_obj_create"]
        fn obj_create(oid: IObjType, par: u64) -> u64;

        #[link_name = "wu_obj_destroy"]
        fn obj_destroy(oid: IObjType, obj: u64) -> u64;

        #[link_name = "wu_obj_get_attr"] // ErrorCode wu_obj_get_attr(void *obj, id_type_t type, void *res, ...);
        fn obj_get_attr(obj: u64, tid: ObjAttr, res: *mut u8, par: ...) -> i32;

        #[link_name = "wu_obj_set_attr"] // ErrorCode wu_obj_set_attr(void *obj, id_type_t type, void *res, ...);
        fn obj_set_attr(obj: u64, tid: ObjAttr, res: *mut u8, par: ...) -> i32;
    }

    use crate::ptr;

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    enum ErrorCode {
        Done = 0,
        OK = 1,
        Error = 2,
    }

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    enum IObjType {
        Obj = 1,
        Button = 10,
    }

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    enum ObjAttr {
        Width = 150,
        Height = 151,
        X = 152,
        Y = 153,
    }

    pub struct Obj {
        oid: IObjType,
        obj: u64,
    }

    impl Obj {
        pub fn new(par: &Obj) -> Self {
            Self {
                oid: IObjType::Obj,
                obj: unsafe { obj_create(IObjType::Obj, par.obj) },
            }
        }

        pub fn from(obj: u64) -> Self {
            Self {
                oid: IObjType::Obj,
                obj,
            }
        }

        pub fn destroy(&self) {
            unsafe { obj_destroy(self.oid, self.obj); }
        }

        pub fn get_width(&self) -> i32 {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Width, ptr!(res)); }
            res
        }

        pub fn get_height(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Height, ptr!(res)); }
            res
        }

        pub fn get_x(&self) -> i32 {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::X, ptr!(res)); }
            res
        }

        pub fn get_y(&self) -> i32 {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Y, ptr!(res)); }
            res
        }

        pub fn set_width(&self, width: i32) {
            unsafe { obj_set_attr(self.obj, ObjAttr::Width, null_mut(), width); }
        }

        pub fn set_height(&self, height: i32) {
            unsafe { obj_set_attr(self.obj, ObjAttr::Height, null_mut(), height); }
        }

        pub fn set_x(&self, x: i32) {
            unsafe { obj_set_attr(self.obj, ObjAttr::X, null_mut(), x); }
        }

        pub fn set_y(&self, y: i32) {
            unsafe { obj_set_attr(self.obj, ObjAttr::Y, null_mut(), y); }
        }
    }
}


pub mod log {
    #[macro_export]
    macro_rules! print {
        ($arg:tt) => {{
            unsafe { _debug($arg.as_ptr(), $arg.len()) }
        }};
        ($($arg:tt)+) => {{
            let s = format!($($arg)+);
            unsafe { _debug(s.as_ptr(), s.len()) }
        }}
    }

    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {{
            print!($($arg)*);
            unsafe { _debug("\n".as_ptr(), 1) }
        }}
    }

    #[macro_export]
    macro_rules! log {
        ($arg:tt) => {{
            unsafe { log_print($arg.as_ptr(), $arg.len()) }
        }};
        ($($arg:tt)+) => {{
            let s = format!($($arg)+);
            unsafe { log_print(s.as_ptr(), s.len()) }
        }}
    }
}

pub mod utils {
    #[macro_export]
    macro_rules! ptr {
        ($p:ident) => { (&mut $p as * mut _ as * mut u8) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
