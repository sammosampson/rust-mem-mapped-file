use std::{
    ffi::*,
    ptr
};

use winapi::{um::{
    winnt::*,
    fileapi::*,
    handleapi::*,
    winbase::*,
    memoryapi::*
}};

fn main() {
    let total_size = 1140850688;
    let file_name = "state.hmi";
    let c_file_name = CString::new(file_name.clone()).unwrap();
    let share_name = "state";
    let c_share_name = CString::new(share_name.clone()).unwrap();
    
    println!("{}", file_name);

    unsafe {
        let file_handle = CreateFileA(
            c_file_name.as_ptr() as *const i8, 
            GENERIC_WRITE|GENERIC_READ, 
            FILE_SHARE_READ | FILE_SHARE_WRITE, 
            ptr::null_mut(), 
            CREATE_ALWAYS, 
            FILE_ATTRIBUTE_NORMAL, 
            ptr::null_mut()
        );

        if file_handle == INVALID_HANDLE_VALUE {
            panic!();
        }

        let mut max_size = LARGE_INTEGER::default();
        *max_size.QuadPart_mut() = total_size as i64;

        //let memory_map = OpenFileMappingA(FILE_MAP_WRITE, 1, c_share_name.as_ptr() as *const i8);
        let memory_map = CreateFileMappingA(
            file_handle, 
            ptr::null_mut(), 
            PAGE_READWRITE, 
            max_size.u().HighPart as u32,
            max_size.u().LowPart as u32, 
            c_share_name.as_ptr() as *const i8
        );

        let err = winapi::um::errhandlingapi::GetLastError();
        println!("{:?}", err);

        if memory_map.is_null() {
            panic!("no memory map")
        }

        let memory_block = MapViewOfFile(
            memory_map, 
            FILE_MAP_ALL_ACCESS,
            0, 
            0, 
            total_size as usize
        );

        if memory_block.is_null() {
            panic!("no memory block")
        }
    }
}
