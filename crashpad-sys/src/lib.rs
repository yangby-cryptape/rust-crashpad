include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(target_os = "windows"))]
const DEFAULT_HANDLER_NAME: &str = "crashpad_handler";
#[cfg(target_os = "windows")]
const DEFAULT_HANDLER_NAME: &str = "crashpad_handler.exe";

#[cfg(all(feature = "with-precompiled", not(target_os = "windows")))]
const HANDLER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bin/crashpad_handler"));
#[cfg(all(feature = "with-precompiled", target_os = "windows"))]
const HANDLER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bin/crashpad_handler.exe"));

/// The default file name of the Crashpad handler executable binary.
pub const fn handler_name() -> &'static str {
    DEFAULT_HANDLER_NAME
}

/// The data of a precompiled Crashpad handler executable binary.
#[cfg(not(feature = "with-precompiled"))]
pub const fn precompiled_handler() -> Option<&'static [u8]> {
    None
}
#[cfg(feature = "with-precompiled")]
pub const fn precompiled_handler() -> Option<&'static [u8]> {
    Some(HANDLER)
}
