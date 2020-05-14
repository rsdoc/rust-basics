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
}
