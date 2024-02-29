// Attribute -> metadata applied to module, crate, item
// Use cases...
// - conditional compilation of code
// - set crate name, version, and type (binary or library)
// - disable lints (warnings)
// - enable compiler features (macros, glob imports, etc)
// - link to foreign library
// - mark functions as unit tests
// - mark functions that will be part of benchmark
// - attribute like macros

// #[outer_attribute] applies to item immediately following it
// examples: functions, module declarations, constants, structures, enums
// Actual example below
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// #![inner_attribute] applies to enclosing item (typically module or crate)
// this attribute is interpreted as applying to the entire scope in which it's placed

#[allow(unused_variables)] // in this case not inner
fn main() {
    let x = 3; // Normally causes warning
}

// Attributes can take args with diff syntaxes
// - #[attribute = "value"]
// - #[attribute(key = "value")]
// - #[attribute(value)]
// - #[attribute(value, value2)]
// - #[attribute(value, value2, value3, value4, value5)]
