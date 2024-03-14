// map() was described as chainable way to simplify `match` statements
// map() on a function returns `Option<T>` results in the nested `Option<Option<T>>`
// Chaining multiple calls together can then become confusing
// That's where another combinator called and_then(), known in some languages as flatmap, comes in

// `and_then()` calls its function input with wrapped value and returns the result
// If `Option` is `None`, returns `None` instead

// In following example, `cookable_v3()` results in `Option<Food>`
// Using map() instead of and_then() would have given `Option<Option<Food>>

#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make sushi
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// We have receipe for everything except Cordon Bleu
fn have_receipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}


// To make dish, need receipe and ingredients
// Can represent logic with chain of `match`es
fn cookable_v1(food: Food) -> Option<Food> {
    match have_receipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v3(food: Food) -> Option<Food> {
    have_receipe(food).and_then(have_ingredients)
}

// Otherwise need to `flatten()` an `Option<Option<Food>>` to get `Option<Food>`
fn cookable_v2(food: Food) -> Option<Food> {
    have_receipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't eat to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
