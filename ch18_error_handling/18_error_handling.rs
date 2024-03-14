// Error handling -> process of handling possiblity of failure
// Failing to read a file then continuing to use bad input would clearly be problematic
// Noticing and explicitly managing errors saves program from pitfalls

// Different ways of handling errors:

// 1) explicit `panic useful for tests and unrecoverable errors, like with functions that haven't been implemented yet, but sometimes the more descriptive unimplemented is better

// 2) `Option` type is for when value is optional or when lack of value not error condition
// When dealing with `Option`, `unwrap` is fine for prototyping and cases where absolutely certain that there is guaranteed to be value
// `expect` is more useful since it lets you specify an error message in case something goes wrong anyway

// 3) `Result` is for when things go wrong and caller has to deal with the proble. You can `unwrap` and `except` as well
