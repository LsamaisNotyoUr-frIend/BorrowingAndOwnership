use std::collections::HashSet;
use std::io;

pub(crate) fn check_word(){
    println!("Enter a word any word");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let optimized_wrd = input.trim();
    let mut is_unique = true;

    // Using hashsets
    {
        let mut seen  = HashSet::new();

        for &letr in optimized_wrd.as_bytes(){
            if !seen.insert(letr){
                is_unique = false;
                break
            }
        }

        if is_unique {
            println!("{} is a unique word", optimized_wrd);
        }else { println!("{} is not unique word", optimized_wrd); }
    }
    is_unique = true;


    // Using fixed boolean arrays
    {
        let mut seen = [false; 256];
        for &letr in optimized_wrd.as_bytes(){
            if seen[letr as usize] {
                is_unique = false;
                break
            }
            seen[letr as usize] = true;
        }

        if is_unique {
            println!("{} is a unique word", optimized_wrd);
        }else { println!("{} is not unique word", optimized_wrd); }
    }
}