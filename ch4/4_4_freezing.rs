// when data is bound by the same name immutably, also freezes
// frozen data cannot be modded until out of scope

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // shadowing by immutable _mutable_integer
        let _mutable_integer = _mutable_integer;

        // Error! _mutable_integer is frozen in this scope
        //_mutable_integer = 50;

        //_mutable integer goes out of scope
    }

    // Ok because no longer frozen, in different scope
    _mutable_integer = 5;
}