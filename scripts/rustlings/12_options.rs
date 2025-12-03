// Exercise 1: Fill the function to return 'None' if 'hour_of_day' is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None
    } else if hour_of_day == 23 {
        Some(0 as u16)
    } else {
        Some(5 as u16)
    }
}

fn main() {
    // Exercise 1:
    let ice_creams = maybe_ice_cream(12);
    // assert_eq!(Some())
}
