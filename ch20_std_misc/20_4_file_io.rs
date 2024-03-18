// `File` struct represents a file that has been opened (it wraps a file descriptor), and gives read/write access to underlying file

// Many things can go wrong when doing file I/O all `File` methods return `io::Result<T>` type which is alias for Result<T, io::Error>

// Failure of all I/O operations are explicit -> which means programmer can see all failure paths and handle them accordinglty