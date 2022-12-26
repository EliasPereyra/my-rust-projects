// Structs is a data structure made of custom variable types

// The traditional look of structs is:
/*
struct RGB {
    red: u8;
    green: u8;
    blue: u8;
}
*/

// And to use the struct:
/*
    let primaryColor = RGB {
        red: 255,
        green: 0,
        blue: 0,
    }
*/

// For accessing a specific value from a struct by just using the dot (primaryColor.red == 255)

struct UserAccount {
    active: bool,
    email: String,
    name: String,
    sign_in_count: i64,
}

pub fn run() {
    let user1 = UserAccount {
        active: true,
        name: String::from("Elias pereyra"),
        email: String::from("myemail@email.com"),
        sign_in_count: 10,
    };

    create_new_user(user1.email, user1.name);
}

fn create_new_user(email: String, name: String) -> UserAccount {
    UserAccount {
        active: true,
        email,
        name,
        sign_in_count: 1,
    }
}
