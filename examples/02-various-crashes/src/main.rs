use std::{env, path, process, thread, time::Duration};

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const MINIDUMP_UPLOAD_URL: &str = "MINIDUMP_UPLOAD_URL";

fn start_crashpad() {
    match env::var(MINIDUMP_UPLOAD_URL) {
        Ok(url) => {
            let cache_dir = path::Path::new("minidump_cache").to_path_buf();
            let started = crashpad::start_crashpad(None, Some(cache_dir), &url).unwrap();
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
}

fn panic_in_rust() {
    panic!("Panic in Rust!")
}

fn segfault_in_rust() {
    #[allow(deref_nullptr)]
    #[allow(clippy::zero_ptr)]
    unsafe {
        *(0 as *mut u32) = 0xDEAD;
    }
}

fn segfault_in_c() {
    unsafe {
        bindings::cause_segfault();
    }
}

fn sleep_millis(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn main() {
    start_crashpad();

    #[cfg(target_os = "linux")]
    crashpad::dump_if_panicked();

    let arg1 = env::args().skip(1).next().unwrap();
    println!("Argument: {}", arg1);

    // a normal work thread
    let _ = thread::Builder::new()
        .name("normal".to_owned())
        .spawn(|| loop {
            sleep_millis(5);
        });

    // a work thread with panic
    let _ = thread::Builder::new()
        .name("poisonous".to_owned())
        .spawn(move || {
            sleep_millis(100);

            match arg1.as_str() {
                "--panic-in-rust" => panic_in_rust(),
                "--segfault-in-rust" => segfault_in_rust(),
                "--segfault-in-c" => segfault_in_c(),
                _ => println!("unknown argument [{}], do nothing.", arg1),
            }
        });

    sleep_millis(5 * 1000);
}
