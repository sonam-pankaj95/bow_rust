use std::collections::HashMap;
use regex::Regex;

fn tokenize(text: &str) -> Vec<&str> {
    // Use regex to tokenize words (you can customize the regex pattern as needed)
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text).map(|m| m.as_str()).collect()
}

fn create_bag_of_words(documents: Vec<&str>) -> HashMap<String, usize> {
    let mut vocabulary: HashMap<String, usize> = HashMap::new();

    for document in documents {
        let tokens = tokenize(document);
        for token in tokens {
            // Count the frequency of words in the vocabulary
            let counter = vocabulary.entry(token.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    vocabulary
}

fn main() {
    // Sample list of documents
    let documents = vec![
        "It was the best of times",
        "it was the worst of times",
        "it was the age of wisdom",
        "it was the age of foolishness",
    ];

    // Create the bag of words representation
    let vocabulary = create_bag_of_words(documents);

    // Print the vocabulary (words and their frequencies)
    for (word, frequency) in &vocabulary {
        println!("Word: {}, Frequency: {}", word, frequency);
    }
}


