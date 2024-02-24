fn main() {
    // This is how a line comment works!
    // Nothing after this is read by the compiler :)

    /*
    This is a block comment,
        /* 
        Which can be nested
        */
    */

    /*
     * You can do this for style
     * But not necessary
     */
    
     // Manipulate expressions with block elements
     let x = 5 + /* 90 + */ 5;
     println!("Is `x` 10 or 100? x = {}", x);
}