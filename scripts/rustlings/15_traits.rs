trait AppendBar {
    fn append_bar(self) -> Self;
}

// Exercise 1: Implement 'AppendBar' trait for strings
impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}

// Exercise 2: Implement 'AppendBar' trait for a vector of strings
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

// Exercise 3: Implementing a trait that can be shared
trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// Don't edit the next two lines
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// Exercise 4: Using traits in signature functions
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

// Exercise 5:
trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // Exercise 1:
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");

    assert_eq!(String::from("Foo").append_bar(), "FooBar");
    assert_eq!(String::from("").append_bar().append_bar(), "BarBar");

    // Exercise 2:
    let mut foo = vec![String::from("Foo")].append_bar();
    assert_eq!(foo.pop().unwrap(), "Bar");
    assert_eq!(foo.pop().unwrap(), "Foo");

    // Exercise 3:
    let licensing_info = "Default license";
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware {
        version_number: "v2.0.0".to_string(),
    };
    assert_eq!(some_software.licensing_info(), licensing_info);
    assert_eq!(some_software.version_number, 1);
    assert_eq!(other_software.licensing_info(), licensing_info);
    assert_eq!(other_software.version_number, "v2.0.0");

    // Exercise 4:
    assert!(compare_license_types(some_software, other_software));

    // Exercise 5:
    assert!(some_func(SomeStruct));
    assert!(some_func(OtherStruct));

    print!("All tests passed!");
}
