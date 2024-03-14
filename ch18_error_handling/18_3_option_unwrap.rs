// What if we except some drink but don't receive one?
// We could test against null string "", but Rust can use compiler to point out cases with no drink

// enum called `Option<T>` in std library is used when absence is possibility
// Manifests as two options:
// 1) `Some(T)` -> element of type T was found
// 2) `None` -> no element was found

// Cases can be explicitly handled with match, implicitly with unwrap
// Implicit handling returns inner element or `panic`

// Possible to manually customize `panic` with expect, but `unwrap` otherwise leaves less meaningful output than explicit handling
// Explicit handling yields more controlled result while retaining option to panic if desired

// Adult handles all drinks, handled explicitly via match
fn give_adult(drink: Option<&str>) {
    // Specify course of action for each case
    match drink {
        Some("lemonade") => println!("Yuck! Sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// Others `panic` before drinking sugary drinks
// Drinks handled implicitly via unwrap
fn drink(drink: Option<&str>) {
    // `unwrap` returns `panic` when it receives a `None`
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaa!!!"); }

    println!("I love {}s!!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}