/*
    -------------------------------------
    Exercise 1: Common struct definitions
    -------------------------------------
*/

struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

/*
    ---------------------
    Exercise 2: Templates
    ---------------------
*/
#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn create_email_order(name: &str, item_number: u32, count: u32) -> Order {
    Order {
        name: String::from(name),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number,
        count,
    }
}

/*
    ---------------------------------------
    Exercise 3: Structs with data and logic
    ---------------------------------------
*/

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            panic!("Can't ship a package with weight below 10 grams.");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    // Exercise 1
    //      Regular color struct
    let green = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };
    assert_eq!(green.red, 0);
    assert_eq!(green.green, 255);
    assert_eq!(green.blue, 0);
    //      Tuple struct
    let red = ColorTupleStruct(255, 0, 0);
    assert_eq!(red.0, 255);
    assert_eq!(red.1, 0);
    assert_eq!(red.2, 0);
    //      Unit struct
    let unit_struct = UnitStruct;
    let message = format!("{unit_struct:?}s are fun!");
    assert_eq!(message, "UnitStructs are fun!");

    // Exercise 2:
    let order_template = create_order_template();
    let your_order = create_email_order("Hacker in Rust", 123, 1);
    dbg!(&your_order);
    assert_eq!(your_order.name, "Hacker in Rust");
    assert_eq!(your_order.year, order_template.year);
    assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
    assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
    assert_eq!(your_order.made_by_email, order_template.made_by_email);
    assert_eq!(your_order.item_number, order_template.item_number);
    assert_eq!(your_order.count, 1);

    // Exercise 3:
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Russia");
    let package = Package::new(sender_country, recipient_country, 1200);
    dbg!(&package);
    assert!(package.is_international());

    let sender_country = String::from("Spain");
    let recipient_country = sender_country.clone();
    let cents_per_gram = 3;
    let package = Package::new(sender_country, recipient_country, 1500);
    dbg!(&package);

    assert!(!package.is_international());
    assert_eq!(package.get_fees(cents_per_gram), 4500);
    assert_eq!(package.get_fees(cents_per_gram * 2), 9000);

    println!("All tests passed!")
}
