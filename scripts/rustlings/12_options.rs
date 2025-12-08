/*
Exercise 1: Fill the function to return:
   - 5 Scoops if the hour is less than 22
   - 0 scoops if the hour is 22 or 23
   - None for any other case
*/
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    }
}

// Exercise 2: Use of if-let and while-let statements.
fn simple_option() {
    let target = "rustlings";
    let optional_target = Some(target);

    // Code for if-let
    if let Some(word) = optional_target {
        assert_eq!(word, target);
    }
}

fn layered_option() {
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }
    let mut cursor = range;

    // Code for while-let
    while let Some(integer) = optional_integers.pop() {
        if let Some(i) = integer {
            assert_eq!(i, cursor);
            cursor -= 1;
        }
    }
    assert_eq!(cursor, 0);
}

fn main() {
    // Exercise 1:
    let icecreams = maybe_icecream(12).unwrap();
    assert_eq!(icecreams, 5);
    assert_eq!(maybe_icecream(0), Some(5));
    assert_eq!(maybe_icecream(9), Some(5));
    assert_eq!(maybe_icecream(18), Some(5));
    assert_eq!(maybe_icecream(22), Some(0));
    assert_eq!(maybe_icecream(23), Some(0));
    assert_eq!(maybe_icecream(24), None);
    assert_eq!(maybe_icecream(25), None);

    // Exercise 2:
    simple_option();
    layered_option();

    println!("All tests passed!");
}
