///
/// 语义项
///
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
