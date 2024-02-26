// sometimes match is awkward, like when matching enums

/*

// Make optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        
        // Need 2 indentations just so we can destructure `i` from the option
        _ => {},
    }
};
*/

// `if let` is much cleaner for this use case
// also adds in extra failure options

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: if `let` destructures `number` into
    // Some(i), evaluate the block (`{}`)
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to a failure case
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition
    let i_like_letters = false;
    
    if let Some(i) = emoticon { 
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
