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
        fn obj_get_attr(obj: u64, tid: ObjAttr, res: *mut u8, par: *const u8) -> i32;

        #[link_name = "wu_obj_set_attr"] // ErrorCode wu_obj_set_attr(void *obj, id_type_t type, void *res, ...);
        fn obj_set_attr(obj: u64, tid: ObjAttr, res: *mut u8, par: *const u8) -> i32;
    }

    pub type CoordType = i32;

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
        Coords = 154,
        OriCoords = 155,
        Rect = 156,
        ScrollCoords = 157,
        ScrollLeft = 158,
        ScrollTop = 159,
        ScrollRight = 160,
        ScrollBottom = 161,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct PosType {
        x: CoordType,
        y: CoordType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct SizeType {
        w: CoordType,
        h: CoordType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct RectType {
        x0: CoordType,
        y0: CoordType,
        x1: CoordType,
        y1: CoordType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct RectSizeType {
        x: CoordType,
        y: CoordType,
        w: CoordType,
        h: CoordType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct BoxType {
        left: CoordType,
        top: CoordType,
        right: CoordType,
        bottom: CoordType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
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

        pub fn get_width(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Width, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_height(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Height, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_x(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::X, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_y(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::Y, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_coords(&self) -> RectType {
            let mut res: RectType = Default::default();
            unsafe { obj_get_attr(self.obj, ObjAttr::Coords, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_ori_coords(&self) -> RectType {
            let mut res: RectType = Default::default();
            unsafe { obj_get_attr(self.obj, ObjAttr::OriCoords, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_rect(&self) -> RectType {
            let mut res: RectType = Default::default();
            unsafe { obj_get_attr(self.obj, ObjAttr::Rect, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_scroll_coords(&self) -> RectType {
            let mut res: RectType = Default::default();
            unsafe { obj_get_attr(self.obj, ObjAttr::ScrollCoords, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_scroll_left(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::ScrollLeft, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_scroll_top(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::ScrollTop, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_scroll_right(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::ScrollRight, ptr!(res), &Default::default()); }
            res
        }

        pub fn get_scroll_bottom(&self) -> CoordType {
            let mut res = 0;
            unsafe { obj_get_attr(self.obj, ObjAttr::ScrollBottom, ptr!(res), &Default::default()); }
            res
        }

        pub fn set_width(&self, width: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::Width, null_mut(), ptr!([width, ]));
            }
        }

        pub fn set_height(&self, height: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::Height, null_mut(), ptr!([height, ]));
            }
        }

        pub fn set_x(&self, x: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::X, null_mut(), ptr!([x, ]));
            }
        }

        pub fn set_y(&self, y: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::Y, null_mut(), ptr!([y, ]));
            }
        }

        pub fn set_coords(&self, coords: RectType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::Coords, null_mut(), ptr!([coords, ]));
            }
        }

        pub fn set_scroll_left(&self, left: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::ScrollLeft, null_mut(), ptr!([left, ]));
            }
        }

        pub fn set_scroll_top(&self, top: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::ScrollTop, null_mut(), ptr!([top, ]));
            }
        }

        pub fn set_scroll_right(&self, right: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::ScrollRight, null_mut(), ptr!([right, ]));
            }
        }

        pub fn set_scroll_bottom(&self, bottom: CoordType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::ScrollBottom, null_mut(), ptr!([bottom, ]));
            }
        }

        pub fn set_scroll_coords(&self, coords: RectType) {
            unsafe {
                obj_set_attr(self.obj, ObjAttr::ScrollCoords, null_mut(), ptr!([coords, ]));
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
        ($p:expr) => { (&mut $p as * mut _ as * mut u8) };
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
}
