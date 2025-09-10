#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn main() {
    let buf = c"Hello, world!\n";
    unsafe {
        libc::write(
            1,
            buf.as_ptr() as *const libc::c_void,
            buf.count_bytes(),
        );
        libc::_exit(0);
    }
}

#[panic_handler]
fn panic_handler(_:&core::panic::PanicInfo) -> ! {
    unsafe {
        libc::_exit(1);
    }
}
