// Strings

// Two types: `String` and `&str`
// `String` = vector of bytes (`Vec<u8>`) but guaranteed to be valid UTF-8 sequence. GROWABLE, HEAP ALLOCATED, NOT NULL TERMINATED
// `&str` is a slice (`&[u8]`) that always points to valid UTF-8 sequence, can be used to view into a `String`, just like `&[T]` is view into `Vec<T>`

/*
fn main() {
    // (all type annotations are superfluous)
    // Reference to string allocated in read-only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert char at end of string
        string.push(c);
        // Insert string at end of string
        string.push_str(", ");
    }

    // Trimmed string is slice to original string, hence no new allocation performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate string
    let alice = String::from("I like dogs");
    // Allocate new memory and store modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
*/

// Literals and escapes

// Multiple ways to write string literals with special chars, all result in similar &str so best to use most convenient form
// Byte string literals result in `[u8; N]`

// Special characters are escaped with backslash character `\`, allows you to add any char to string, even unprintable ones and ones you don't know how to type
// Literal backslash -> `\\`

// String or character delimiters must be escaped "\"" or '\''

/*
fn main() {
    // Can use escapes to wrtie bytes by hexadecimal values
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+2111D) is called {}",
        unicode_codepoint, character_name);

    let long_string = "String literals
                       can span multiple lines.\
                       Linebreak and indentation here ->\
                       <- can be escaped too!";
    
    println!("{}", long_string);
}
*/

// Raw string literals

// When too many characters need to be escaped, better justto write string out as-is
/*
fn main() {
    let raw_str = r"Escapes don't work here:\x3F \u{211D}";
    println!("{}", raw_str);

    // If you need quotes in a raw string, add pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in delimiter
    // You can use up to 65535 #s
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
*/

use std::str;

fn main() {
    // Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays do not have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // No unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Converting byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings do not have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // Cannot always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}