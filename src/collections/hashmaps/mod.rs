//! Hashmaps -
//! - key value pairs
use std::collections::HashMap;

pub fn run() {
    // creating HashMap
    let mut scores: HashMap<String, String> = HashMap::new();

    // adding data to hashmap
    scores.insert(String::from("Rahul"), 100.to_string());
    scores.insert(String::from("Raj"), 120.to_string());
    scores.insert(String::from("Sonu"), 500.to_string());

    println!(" the hash map value is {:?}", scores);

    // construct hashmap with collect() method
    let team = vec!["Rahul", "Sonu", "Raa"];
    let score = vec![100, 200, 500];

    let new_scores: HashMap<_, _> = team.iter().zip(score.iter()).collect();

    // new_score will be unordered hashmap
    println!(" the new scores is {:?}", new_scores);

    // accessing values in hashmap -> get() method
    let rahul_score = scores.get("Rahul"); // always returns variants of Option enum
    println!(" the score value of rahul is {:?}", rahul_score);

    // using for loop
    for (key, value) in &new_scores {
        println!(" scores key & value : {:?} {:?}", key, value);
    }

    // updating values in hashmap
    // we have to override the values using insert() method
    scores.insert(String::from("Rahul"), 1000.to_string());

    println!(" new value of rahul is : {:?}", scores.get("Rahul"));
    // using entry() -> if exists update or create a new entry
    scores
        .entry(String::from("Raja"))
        .or_insert(2000.to_string());

    println!(" new value of raja is : {:?}", scores.get("Raja"));
}
