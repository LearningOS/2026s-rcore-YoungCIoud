//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::PAGE_SIZE,
    mm::{
        PTEFlags,
        translated_byte_buffer
    },
    task::{
        ask_current_syscall_cnt, change_program_brk, current_mmap, current_user_token, exit_current_and_run_next, suspend_current_and_run_next
    },
    timer::get_time_us,
    tools::{
        read_data_buffers, write_data_buffers
    }
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();

    let tv = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let ptr = ts as *mut u8;
    let len = size_of::<TimeVal>();
    if let Some(buffers) = translated_byte_buffer(
        current_user_token(),
        ptr,
        len,
        PTEFlags::U | PTEFlags::W,
    ) {
        write_data_buffers(tv, buffers);
        0
    } else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    match trace_request {
        0 => {
            // trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值
            if let Some(buffers) = translated_byte_buffer(
                current_user_token(),
                id as *const u8,
                1,
                PTEFlags::U | PTEFlags::R
            ) {
                read_data_buffers::<u8>(&buffers) as isize
            } else {
                -1
            }
        }
        1 => {
            // trace_request 为 1，则 id 应被视作 *mut u8 ，表示写入 data
             if let Some(buffers) = translated_byte_buffer(
                current_user_token(),
                id as *const u8,
                1,
                PTEFlags::U | PTEFlags::W
            ) {
                write_data_buffers(data as u8, buffers);
                0
            } else {
                -1
            }
        }
        2 => {
            ask_current_syscall_cnt(id) as isize
        }
        _ => { panic!("Unsupported trace_request: {}", trace_request) }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if (start % PAGE_SIZE != 0) || (prot & !0x7 != 0) || (prot & 0x7 == 0) {
        return -1;
    }
    let len = if len % PAGE_SIZE != 0 {
        len + PAGE_SIZE - len % PAGE_SIZE
    } else {
        len
    };

    current_mmap(start, len, prot)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
