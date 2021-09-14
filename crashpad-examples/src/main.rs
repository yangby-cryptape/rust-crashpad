use std::{env, process};

const MINIDUMP_UPLOAD_URL: &str = "MINIDUMP_UPLOAD_URL";

fn main() {
    match env::var(MINIDUMP_UPLOAD_URL) {
        Ok(url) => {
            let started = crashpad::start_crashpad(None, None, &url).unwrap();
            println!("Crashpad: {}", started);
        }
        Err(err) => {
            println!(
                "couldn't find the minidump upload url \
                from the environment variable \"{}\" since {}",
                MINIDUMP_UPLOAD_URL, err
            );
            process::exit(1);
        }
    };
    #[allow(deref_nullptr)]
    #[allow(clippy::zero_ptr)]
    unsafe {
        *(0 as *mut u32) = 100;
    }
}
