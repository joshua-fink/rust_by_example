// Closures as input parameters are possible, so returning closures
// as output parameters should also be possible
// However, anonymous closure types are by definition unknown so we have to use impl Trait to return them

// The valid traits for returning a closure are:
// Fn, FnMut, FnOnce

// Beyond this, move keyword must be use, signals all captures occur by value
// Required because any captures by referenece dropped as soon as function exited
// leaving invalid references in closure

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}