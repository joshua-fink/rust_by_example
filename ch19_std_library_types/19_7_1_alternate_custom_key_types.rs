// Alternate/custom key types

// Any type that implements the `Eq` and `Hash` traits can be a key in HashMap. This includes:
// - bool (not useful because T/F)
// - int, uint, all variations thereof
// - `String` and `&str`

// f32 and f64 do not implement Hash due to floating point precision errors

// All collection classes implement Eq and Hash if contained type also respectively implements Eq and Hash. For example, Vec<T> will implement Hash if T implements Hash

// You can easily implement Eq and Hash for custom type with one line: #[derive(PartialEq, Eq, Hash)]

// Compiler will do the rest, more control over the details, can implement Eq or Hash yourself, guide will not cover specifics of implementing Hash

use std::collections::HashMap;

// Eq requires that you derive PartialEq on type
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
                 username: &'a str, password: &'a str){
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login failed"),
    }
}

fn main(){
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");
}