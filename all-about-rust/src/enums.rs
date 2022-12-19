// Enums are types which have a few definite values

enum Movement {
    Fly_up,
    Fly_down,
    Fly_right,
    Fly_left,
}

pub fn run() {
    let plane_goes_up = Movement::Fly_up;
    let plane_goes_down = Movement::Fly_down;
    let plane_goes_right = Movement::Fly_right;
    let plane_goes_left = Movement::Fly_left;

    move_plane(plane_goes_up);
    move_plane(plane_goes_down);
    move_plane(plane_goes_right);
    move_plane(plane_goes_left);
}

fn move_plane(m: Movement) {
    // Perform action depending on data received
    match m {
        Movement::Fly_up => println!("The plane is flying up"),
        Movement::Fly_down => println!("The plane is flying down"),
        Movement::Fly_right => println!("The plane is flying to the right"),
        Movement::Fly_left => println!("The plane is flying to the left"),
    }
}
