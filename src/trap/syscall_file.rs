/// Read n bytes from a file and put it into a
/// a buffer.
/// # Wrapper 
/// `ssize_t read(int fd, void *buf, size_t n)`
pub fn sys_read() -> usize {
  0
}

/// Write n bytes from a buffer to a file.
/// # Wrapper
/// `ssize_t write(int fd, const void *buf, size_t n)` 
pub fn sys_write() -> usize {
  0
}

/// Open and possibly create a file or device.
/// # Wrapper
/// `int open(const char *file, int flags);` 
pub fn sys_open() -> usize {
  0
}

/// Close a file descriptor.
/// # Wrapper 
/// `int close(int fd)`
pub fn sys_close() -> usize {
  0
}

/// Create pipe.
/// # Wrapper
/// `int pipe(int p[2]);`
pub fn sys_pipe() -> usize {
  0
}

/// Return a new file descriptor referring to 
/// the a file.
/// # Wrapper
/// `int dup(int fd)`
pub fn sys_dup() -> usize {
  0
}
