//---------------------------------------------------------------------------------------------------- umask
/// Set the [`umask`](https://man7.org/linux/man-pages/man2/umask.2.html) value of your entire process.
///
/// This will be the permission `umask` used by `disk` (and your entire process) when creating directories/files.
///
/// Note, this is `umask`, not `chmod`, meaning, to get the equivalent behavior of:
/// ```txt
/// chmod 640
/// ```
/// you should set a `umask` of:
/// ```txt
/// umask 037
/// ```
/// which in Rust, using this function with octal notation, would look like:
/// ```rust
/// fn main() {
///     // Make all future writes use this `umask`.
///     disk::umask(0o037);
///
///    /* write some files with 640 (rw-r-----) perms */
///    /* create directories with 740 (rwxr-----) perms */
/// }
/// ```
/// The `umask` should take care to make sure that directory executable
/// bits are not masked. `umask` cannot _add_ executable bits so even with
/// `0o000`, files that did not already have it cannot gain them.
///
/// ## Default values
/// These are the default values that will be subtracted from when using this function.
///
/// - The default `Rust` directory permissions are `0o755` or `rwxr-xr-x`
/// - The default `Rust` file permissions are `0o666` or `rw-r--r--`
///
/// ## Examples
/// | Desired permissions | `chmod` equivalent | `umask()` |
/// |---------------------|--------------------|-----------|
/// | `rwxr-----`         | `chmod 755`        | `umask(0o022)`
/// | `rwxrwx---`         | `chmod 770`        | `umask(0o007)`
/// | `rwxrw----`         | `chmod 760`        | `umask(0o017)`
/// | `rwxr-x---`         | `chmod 750`        | `umask(0o027)`
/// | `rwxr-----`         | `chmod 740`        | `umask(0o037)`
/// | `rwx------`         | `chmod 740`        | `umask(0o077)`
///
/// ## Note
/// This function does nothing on non-UNIX targets (Windows).
pub fn umask(umask: u32) {
	#[cfg(target_family = "unix")]
	unsafe { libc::umask(umask as libc::mode_t); }
}
