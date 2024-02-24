fn main() {
    // vars can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // or default is used
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // a type can be inferred from context
    let mut inferred_type = 12; // type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // a mutable variable's value can be changed
    let mut mutable = 12; // 'Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed
    // mutable = true;
    
    // Variables can be overwritten with shadowing
    let mutable = true;
}