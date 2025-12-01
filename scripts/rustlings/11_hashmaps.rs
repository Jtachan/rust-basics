use std::collections::HashMap;
use std::iter::FromIterator;

// Exercise 1: Create a HashMap with three items and a sum of their values greater than 5.
fn basket_template() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 5);
    basket.insert(String::from("pear"), 3);
    basket
}

/*
    Exercise 2: Update a currently existing basket to insert all the new fruits they weren't
    before. The total number of elements (values) must be 11 or greater.
*/
#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn get_base_fruit_basket() -> HashMap<Fruit, u32> {
    let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
    HashMap::from_iter(content)
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        basket.entry(fruit).or_insert(2);
    }
}

/*
   Exercise 3: Build a score table for soccer (football) matches.
*/

#[derive(Default, Debug)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(result: &str) -> HashMap<&str, TeamScores> {
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in result.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Populate the score table
        let team_record = scores.entry(team_1_name).or_insert(TeamScores {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_record.goals_scored += team_1_score;
        team_record.goals_conceded += team_2_score;

        let team_record = scores.entry(team_2_name).or_insert(TeamScores {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_record.goals_scored += team_2_score;
        team_record.goals_conceded += team_1_score;
    }

    scores
}

const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

fn main() {
    // Exercise 1
    let basket = basket_template();
    dbg!(&basket); // Note: the hashmap is unordered
    assert!(basket.len() == 3);
    assert!(basket.values().sum::<u32>() >= 5);

    // Exercise 2
    let mut basket = get_base_fruit_basket();
    fruit_basket(&mut basket);
    dbg!(&basket);

    assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
    assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
    assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    assert!(basket.len() >= 5);
    assert!(basket.values().sum::<u32>() >= 11);

    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];
    for fruit_kind in fruit_kinds {
        let Some(amount) = basket.get(&fruit_kind) else {
            panic!("Fruit kind {:?} was not found in basket", fruit_kind);
        };
        assert!(*amount > 0);
    }

    // Exercise 3
    let scores = build_scores_table(RESULTS);
    dbg!(&scores);

    assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
        .iter()
        .all(|team_name| scores.contains_key(team_name)));

    let team = scores.get("England").unwrap();
    assert_eq!(team.goals_scored, 6);
    assert_eq!(team.goals_conceded, 4);

    let team = scores.get("Spain").unwrap();
    assert_eq!(team.goals_scored, 0);
    assert_eq!(team.goals_conceded, 3);

    println!("All tests passed!");
}
