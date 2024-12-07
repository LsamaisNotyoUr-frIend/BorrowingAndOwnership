use std::io;

fn count_words() {
    println!("Write a simple sentence with multiple words");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let optimized_wrd = input.trim();
    let mut word_count: i32 = 0;

    {
        for (inx, &wrd) in optimized_wrd.as_bytes().iter().enumerate() {
            if wrd == b' ' {
                word_count += 1
            }
        }

        if !optimized_wrd.is_empty() {
            word_count += 1;
        }

        println!("The word count is {}", word_count);
    }
    word_count = 0;
    {
        for (inx, wrd) in optimized_wrd.chars().enumerate() {
            if wrd == ' ' {
                word_count += 1;
            }
        }
        if !optimized_wrd.is_empty() {
            word_count += 1;
        }

        println!("The word count is {}", word_count);
    }
    word_count = 0;

    {
        let words = optimized_wrd.split_whitespace();
        let word_count = words.count();

        println!("The word count is {}", word_count);
    }
}