
//! Some useful tool functions

use core::mem::{MaybeUninit, size_of};

use alloc::vec::Vec;

/// write data to the mutable u8 Vec
pub fn write_data_buffers<T: Copy>(data: T, buffers: Vec<&mut [u8]>) {
    let src = unsafe {
        core::slice::from_raw_parts(&data as *const T as *const u8, size_of::<T>())
    };

    let mut cur = 0;
    for buffer in buffers {
        let len = buffer.len();
        buffer.copy_from_slice(&src[cur..cur + len]);
        cur += len;
    }

    assert_eq!(cur, size_of::<T>());
}

/// read data from a mutable u8 Vec
pub fn read_data_buffers<T: Copy>(buffers: &Vec<&mut [u8]>) -> T {
    let mut data = MaybeUninit::<T>::uninit();
    let dst = unsafe {
        core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, size_of::<T>())
    };

    let mut cur= 0;
    for buffer in buffers {
        let len = buffer.len();
        dst[cur..cur + len].copy_from_slice(&buffer[..]);
        cur += len;
    }
    
    assert_eq!(cur, size_of::<T>());

    unsafe { data.assume_init() }
}
