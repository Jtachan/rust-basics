use std::fmt::{Display, Formatter};
use std::rc::Rc;

// Exercise 1 - Smart pointers as `Box<T>`
// ---------------------------------------
/* Fill both 'create list' functions to run correctly the tests. */
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>), // Indirect reference of a pointer to the next list.
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(2, Box::new(List::Cons(1, Box::new(List::Nil))))
}

// Exercise 2 - Reference Counter pointer `Rc`
#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupyter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Display for Planet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Planet::Mercury(_) => write!(f, "Mercury"),
            Planet::Venus(_) => write!(f, "Venus"),
            Planet::Earth(_) => write!(f, "Earth"),
            Planet::Mars(_) => write!(f, "Mars"),
            Planet::Jupyter(_) => write!(f, "Jupyter"),
            Planet::Saturn(_) => write!(f, "Saturn"),
            Planet::Uranus(_) => write!(f, "Uranus"),
            Planet::Neptune(_) => write!(f, "Neptune"),
        }
    }
}

impl Planet {
    fn sun_rc(&self) -> &Rc<Sun> {
        match self {
            Planet::Mercury(rc) => rc,
            Planet::Venus(rc) => rc,
            Planet::Earth(rc) => rc,
            Planet::Mars(rc) => rc,
            Planet::Jupyter(rc) => rc,
            Planet::Saturn(rc) => rc,
            Planet::Uranus(rc) => rc,
            Planet::Neptune(rc) => rc,
        }
    }

    fn details(&self) {
        println!("Hi from {self:?}!");
        println!("Reference count = {}\n", Rc::strong_count(self.sun_rc()));
    }
}

fn plante_travel() {
    // The full body for exercise 2:
    let sun = Rc::new(Sun);
    println!("We start at the Sun!");
    println!("Reference count = {}\n", Rc::strong_count(&sun));

    let mercury = Planet::Mercury(Rc::clone(&sun));
    mercury.details();
    {
        let venus = Planet::Venus(Rc::clone(&sun));
        venus.details();
        {
            let earth = Planet::Earth(Rc::clone(&sun));
            earth.details();
            {
                let mars = Planet::Mars(Rc::clone(&sun));
                mars.details();
                {
                    let jupiter = Planet::Jupyter(Rc::clone(&sun));
                    jupiter.details();
                    {
                        let saturn = Planet::Saturn(Rc::clone(&sun));
                        saturn.details();
                        {
                            let uranus = Planet::Uranus(Rc::clone(&sun));
                            uranus.details();
                            {
                                let neptune = Planet::Neptune(Rc::clone(&sun));
                                neptune.details();
                                assert_eq!(Rc::strong_count(&sun), 9);
                            }
                            println!("Back to Uranus");
                            assert_eq!(Rc::strong_count(&sun), 8);
                        }
                        println!("Back to Saturn");
                        assert_eq!(Rc::strong_count(&sun), 7);
                    }
                    println!("Back to Jupyter");
                    assert_eq!(Rc::strong_count(&sun), 6);
                }
                println!("Back to Mars");
                assert_eq!(Rc::strong_count(&sun), 5);
            }
            println!("Back to Earth");
            assert_eq!(Rc::strong_count(&sun), 4);
        }
        println!("Back to Venus");
        assert_eq!(Rc::strong_count(&sun), 3);
    }
    println!("Back to Mercury");
    assert_eq!(Rc::strong_count(&sun), 2);

    drop(mercury);
    assert_eq!(Rc::strong_count(&sun), 1);
}

// Test area:
fn main() {
    // Exercise 1:
    println!("\nExercise 1:");
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );

    // Exercise 2:
    println!("\nExercise 2:");
    plante_travel();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1:
    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }

    #[test]
    fn reference_counter() {
        plante_travel();
    }
}
