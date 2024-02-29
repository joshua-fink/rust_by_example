// like if let, while let makes awkward match sequences more tolerable

/*
WITHOUT WHILE LET

// Make `optional` of type `Option<i32>`
let mut optional = Some(0);

// Repeatedly try this test
loop {
    match optional {
        // If optional destructures, evaluate this block
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        },
        // Quit the loop when destructure fails:
        _ => { break; }
        // Why is this necessary, needs to be better way!
    }
}
*/

fn main() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Reads: while `let` destructures `optional` into `Some(i)`,
    // evaluate the block (`{}`) else break;
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // if let has additional optional else/else if clauses
    // while let does not have these
}