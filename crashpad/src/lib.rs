use std::{fs::OpenOptions, io::Write as _, path::PathBuf, process};

pub mod error;

use error::{Error, Result};

fn convert_to_c_chars<S>(input: S) -> Vec<libc::c_char>
where
    S: ToString,
{
    let mut c_chars = input
        .to_string()
        .into_bytes()
        .into_iter()
        .map(|c| c as libc::c_char)
        .collect::<Vec<_>>();
    c_chars.push(0);
    c_chars
}

/// Starts a Crashpad handler process, performing any necessary handshake to configure it.
///
/// This function is a wrapper of `crashpad::CrashpadClient::StartHandler()`.
///
/// Parameters:
/// - `handler_opt`: The path to a Crashpad handler executable.
///   If this parameter is not provided, the function will try to find the default handler in `PATH`.
/// - `data_dir_opt`: The path to the Crashpad data directory.
///   The handler will be started with this path as its `--database` argument and its
///   `--metrics-dir` argument.
///   If this parameter is not provided, the function will create a temporary directory instead of.
/// - `url`: The URL of an upload server.
///   The handler will be started with this URL as its `--url` argument.
///
/// Returns: true on success, false on failure.
pub fn start_crashpad(
    handler_opt: Option<PathBuf>,
    data_dir_opt: Option<PathBuf>,
    url: &str,
) -> Result<bool> {
    let data_dir = if let Some(data_dir) = data_dir_opt {
        data_dir
    } else {
        let pid = process::id();
        let prefix = format!("crashpad-cache-{}-", pid);
        tempfile::Builder::new()
            .prefix(&prefix)
            .suffix(".tmp")
            .rand_bytes(6)
            .tempdir()
            .map(tempfile::TempDir::into_path)
            .map_err(|err| {
                let errmsg = format!(
                    "failed to create a temp dir for crashpad data, since {}",
                    err
                );
                Error::Other(errmsg)
            })?
    };

    let handler = if let Some(handler) = handler_opt {
        if handler.exists() {
            handler.canonicalize().map_err(|err| {
                let errmsg = format!(
                    "`{}` couldn't be converted to a canonical and absolute path since {}",
                    handler.display(),
                    err
                );
                Error::Handler(errmsg)
            })
        } else {
            let errmsg = format!("`{}` doesn't exist", handler.display());
            Err(Error::Handler(errmsg))
        }
    } else {
        let handler_name = crashpad_sys::handler_name();
        which::which(handler_name)
            .map_err(Error::Which)
            .or_else(|err| {
                if let Some(data) = crashpad_sys::precompiled_handler() {
                    let bin_file = data_dir.join(handler_name);
                    if bin_file.exists() {
                        return Ok(bin_file);
                    }
                    OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&bin_file)
                        .map_err(|err| {
                            let errmsg = format!(
                                "failed to create a temporary binary `{}` since {}",
                                bin_file.display(),
                                err
                            );
                            Error::Handler(errmsg)
                        })
                        .and_then(|mut file| {
                            file.write_all(data).map_err(|err| {
                                let errmsg = format!(
                                    "failed to write into the temporary file `{}` since {}",
                                    bin_file.display(),
                                    err
                                );
                                Error::Handler(errmsg)
                            })?;
                            let mut permissions = file
                                .metadata()
                                .map_err(|err| {
                                    let errmsg = format!(
                                        "failed to query the temporary file `{}` since {}",
                                        bin_file.display(),
                                        err
                                    );
                                    Error::Handler(errmsg)
                                })?
                                .permissions();
                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt as _;
                                permissions.set_mode(0o555);
                            }
                            #[cfg(not(unix))]
                            {
                                permissions.set_readonly(true);
                            }
                            file.set_permissions(permissions).map_err(|err| {
                                let errmsg = format!(
                                    "failed to set permissions of \
                                     the temporary file `{}` since {}",
                                    bin_file.display(),
                                    err
                                );
                                Error::Handler(errmsg)
                            })
                        })?;
                    Ok(bin_file)
                } else {
                    Err(err)
                }
            })
    }?;

    log::trace!("Crashpad Handler: {}", handler.display());
    log::trace!("Crashpad Data Dir: {}", data_dir.display());
    log::trace!("Crashpad Upload Url: {}", url);

    let status = {
        let mut handler = convert_to_c_chars(handler.display());
        let mut data_dir = convert_to_c_chars(data_dir.display());
        let mut url = convert_to_c_chars(url);
        unsafe {
            crashpad_sys::start_crashpad(
                handler.as_mut_ptr(),
                data_dir.as_mut_ptr(),
                url.as_mut_ptr(),
            )
        }
    };

    Ok(status)
}

/// Creates a dump through sending a signal `SIGQUIT` to it when panics happened.
#[cfg(target_os = "linux")]
pub fn dump_if_panicked() {
    use std::panic;
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        default_panic(panic_info);
        unsafe { libc::raise(libc::SIGQUIT) };
    }));
}
