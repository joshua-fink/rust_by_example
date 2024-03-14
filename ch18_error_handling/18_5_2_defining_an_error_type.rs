// Defining an error type

// Sometimes simplier code-wise to mask all different error with single type of error -> create custom error

// "Good" error type
// - Represents different error with same type
// - Presents nice error messages to user
// - Easy to compare with other types
// -- Good -> Err(EmptyVec)
// -- Bad -> Err("Pleases use vec with at least one element".to_owned())

// - Can hold info about error
// -- Good: Err(BadChar(c, position))
// -- Bad: Err("+ cannot be used here".to_owned())

// - Composes well with other errors

