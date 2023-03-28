// Enums are types which have a few definite values

enum Movement {
    FlyUp,
    FlyDown,
    FlyRight,
    FlyLeft,
}

enum KeyPressed {
    KeyUp,
    KeyDown,
    KeyRight,
    KeyLeft,
}

fn move_plane(m: Movement) {
    // Perform action depending on data received
    match m {
        Movement::FlyUp => println!("The plane is flying up"),
        Movement::FlyDown => println!("The plane is flying down"),
        Movement::FlyRight => println!("The plane is flying to the right"),
        Movement::FlyLeft => println!("The plane is flying to the left"),
    }
}

fn press_key(k: KeyPressed) {
    match k {
        KeyPressed::KeyUp => println!("Key up pressed"),
        KeyPressed::KeyDown => println!("Key down pressed"),
        KeyPressed::KeyRight => println!("Key right pressed"),
        KeyPressed::KeyLeft => println!("Key left pressed"),
    }
}

pub fn run() {
    let plane_goes_up = Movement::FlyUp;
    let plane_goes_down = Movement::FlyDown;
    let plane_goes_right = Movement::FlyRight;
    let plane_goes_left = Movement::FlyLeft;

    let key_pressed_up = KeyPressed::KeyUp;
    let key_pressed_down = KeyPressed::KeyDown;
    let key_pressed_right = KeyPressed::KeyRight;
    let key_pressed_left = KeyPressed::KeyLeft;

    move_plane(plane_goes_up);
    move_plane(plane_goes_down);
    move_plane(plane_goes_right);
    move_plane(plane_goes_left);

    press_key(key_pressed_up);
    press_key(key_pressed_down);
    press_key(key_pressed_right);
    press_key(key_pressed_left);
}
