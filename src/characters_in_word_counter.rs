use std::io;
use std::collections::HashMap;


pub(crate) fn count_chars(){
    println!("Write a Word any word and get the number of letter that it has or a sentence");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let optimized_wrd = input.trim().to_lowercase();
    let mut ordered_words: HashMap<char, i32> = HashMap::new();

    for ch in optimized_wrd.chars() {
        if ch.is_alphabetic(){
            let count = ordered_words.entry(ch).or_insert(0);
            *count += 1;
        }
    }

    let mut sorted_chars: Vec<_> = ordered_words.into_iter().collect();
    sorted_chars.sort_by(|a, b| b.1.cmp(&a.1));

    println!("the word {} has the letters", input.trim());
    let mut total_count = 0;
    for (word, count) in sorted_chars {
        total_count += count;
        print!("{};{}  ", word, count);
    }
    println!("\nFor a total of {}", total_count)
}