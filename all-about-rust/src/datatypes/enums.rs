// Enums are types which have a few definite values

enum Movement {
    FlyUp,
    FlyDown,
    FlyRight,
    FlyLeft,
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

pub fn run() {
    let plane_goes_up = Movement::FlyUp;
    let plane_goes_down = Movement::FlyDown;
    let plane_goes_right = Movement::FlyRight;
    let plane_goes_left = Movement::FlyLeft;

    move_plane(plane_goes_up);
    move_plane(plane_goes_down);
    move_plane(plane_goes_right);
    move_plane(plane_goes_left);
}
