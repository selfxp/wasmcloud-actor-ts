// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
const _: () = {
  
  #[doc(hidden)]
  #[export_name = "hello-world"]
  #[allow(non_snake_case)]
  unsafe extern "C" fn __export_hello_world() -> i32 {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    
    // Before executing any other code, use this function to run all static
    // constructors, if they have not yet been run. This is a hack required
    // to work around wasi-libc ctors calling import functions to initialize
    // the environment.
    //
    // This functionality will be removed once rust 1.69.0 is stable, at which
    // point wasi-libc will no longer have this behavior.
    //
    // See
    // https://github.com/bytecodealliance/preview2-prototyping/issues/99
    // for more details.
    #[cfg(target_arch="wasm32")]
    wit_bindgen::rt::run_ctors_once();
    
    let result0 = <_GuestImpl as Guest>::hello_world();
    let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
    let vec2 = (result0.into_bytes()).into_boxed_slice();
    let ptr2 = vec2.as_ptr() as i32;
    let len2 = vec2.len() as i32;
    ::core::mem::forget(vec2);
    *((ptr1 + 4) as *mut i32) = len2;
    *((ptr1 + 0) as *mut i32) = ptr2;
    ptr1
  }
  
  const _: () = {
    #[doc(hidden)]
    #[export_name = "cabi_post_hello-world"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_hello_world(arg0: i32,) {
      let l0 = *((arg0 + 0) as *const i32);
      let l1 = *((arg0 + 4) as *const i32);
      wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
    }
  };
};
use super::Component as _GuestImpl;
pub trait Guest {
  fn hello_world() -> wit_bindgen::rt::string::String;
}

#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, vec::Vec, string::String};

#[repr(align(4))]
struct _RetArea([u8; 8]);
static mut _RET_AREA: _RetArea = _RetArea([0; 8]);

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:hello"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 173] = [3, 0, 5, 104, 101, 108, 108, 111, 0, 97, 115, 109, 13, 0, 1, 0, 7, 52, 1, 65, 2, 1, 65, 2, 1, 64, 0, 0, 115, 4, 0, 11, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 1, 0, 4, 1, 20, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 114, 117, 115, 116, 47, 104, 101, 108, 108, 111, 4, 0, 11, 11, 1, 0, 5, 104, 101, 108, 108, 111, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
