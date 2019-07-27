extern crate libc;
use libc::c_int;

use std::mem;
use std::ptr;

#[link(name = "vector_add", kind = "static")]
extern "C" {
    fn vectorAdd_main(hc: *mut c_int);
}

fn main() {
    let n = 50000;
    // unsafe {
    //     let ptr: *mut i32;
    //     ptr = libc::malloc(mem::size_of::<i32>() * 50000) as *mut i32;
    //     if ptr.is_null() {
    //         panic!("failed to allocate memory");
    //     }
    //     vectorAdd_main(ptr);
    //     println!("{}", *(ptr));
    //     libc::free(ptr as *mut libc::c_void);
    // }
    let mut v = vec![0; n];
    unsafe{
        vectorAdd_main(v.as_mut_ptr());
    }
    let mut i = 0;
    while i < 10 {
        println!("{}", v[i]);
        i = i + 1;
    }
}
