use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        
        // Question mark checks if it errors, returns error else continues
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            // for every element except first, add comma 
            // Use ? to return on errors
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}