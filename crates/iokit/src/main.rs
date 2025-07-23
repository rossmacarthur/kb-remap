use std::ffi::{c_void, CString};
use std::ptr;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopAddSource(cfLoop: *mut c_void, source: *mut c_void, mode: *mut c_void);
    fn CFRunLoopRun();
    static kCFRunLoopDefaultMode: *mut c_void;
}

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceAddMatchingNotification(
        notifyPort: *mut c_void,
        notificationType: *const i8,
        matching: *mut c_void,
        callback: extern "C" fn(ref_con: *mut c_void, iterator: *mut c_void),
        refCon: *mut c_void,
        notification: *mut *mut c_void,
    ) -> i32;

    fn IOServiceMatching(name: *const i8) -> *mut c_void;
    fn IONotificationPortCreate(masterPort: u32) -> *mut c_void;
    fn IONotificationPortGetRunLoopSource(notify: *mut c_void) -> *mut c_void;
    fn IOIteratorNext(iterator: *mut c_void) -> u32;
    fn IOObjectRelease(object: u32) -> i32;
}

extern "C" fn device_added(_ref_con: *mut c_void, iterator: *mut c_void) {
    unsafe {
        loop {
            let service = IOIteratorNext(iterator);
            if service == 0 {
                break;
            }
            println!("USB device added!");
            IOObjectRelease(service);
        }
    }
}

fn main() {
    println!("started...");
    // Keep the CStrings alive for the duration of the use.
    let iousbdevice_str = CString::new("IOUSBDevice").unwrap();
    let ioservicematched_str = CString::new("IOServiceMatched").unwrap();

    unsafe {
        let notify_port = IONotificationPortCreate(0);
        if notify_port.is_null() {
            panic!("Failed to create notification port");
        }

        let run_loop_source = IONotificationPortGetRunLoopSource(notify_port);
        if run_loop_source.is_null() {
            panic!("Failed to get run loop source");
        }

        let run_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(run_loop, run_loop_source, kCFRunLoopDefaultMode);

        let matching_dictionary = IOServiceMatching(iousbdevice_str.as_ptr());
        if matching_dictionary.is_null() {
            panic!("Failed to create matching dictionary");
        }

        let mut notification: *mut c_void = ptr::null_mut();
        let result = IOServiceAddMatchingNotification(
            notify_port,
            ioservicematched_str.as_ptr(),
            matching_dictionary,
            device_added,
            ptr::null_mut(),
            &mut notification,
        );

        if result != 0 {
            panic!(
                "Failed to add matching notification. Error code: {}",
                result
            );
        }

        // Iterate through existing devices
        device_added(ptr::null_mut(), notification);

        println!("Listening for USB device events...");
        CFRunLoopRun();
    }
}
