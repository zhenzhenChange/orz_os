///
/// 语义项
///
use core::panic::PanicInfo;

use crate::kits::sbi;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[ORZ_OS_SEMANTICS] panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }

    sbi::shutdown(true)
}
