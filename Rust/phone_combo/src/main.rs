fn main() {
    println!("Hello, world!");
}

fn letter_combinations(digits: String) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut string_array: Vec<String> = Vec::new();

    for i in 0..chars.len()  {
        let current_char: char = chars[i];
        letter_vec: Vec<char> = digit_char_to_letter_vec(current_char);
        if string_array.len() == 0  {
            for j in letter_vec.len()  {
                string_array.push(letter_vec[j]);
            }
        }  else  {
            string_array = apply_char_vec_to_string_vec(string_array, letter_vec);
        }
    }
    return string_array;
}


fn apply_char_vec_to_string_vec(string_vec, char_vec)  {
    let mut new_string_vec: Vec<String> = Vec::new();
    for i in 0..string_vec.len()  {
        let current_string = string_vec[i];
        for j in 0..char_vec.len()  {
            let current_char = char_vec[j];
            let new_string = current_string + current_char;
            new_string_vec.push(new_string);
        }
    }
    return new_string_vec;
}



fn digit_char_to_letter_vec(input: char) -> Vec<char>  {
    match input  {
        '2' => return ['a', 'b', 'c'],
        '3' => return ['d', 'e', 'f'],
        '4' => return ['g', 'h', 'i'],
        '5' => return ['j', 'k', 'l'],
        '6' => return ['m', 'n', 'o'],
        '7' => return ['p', 'q', 'r', 's'],
        '8' => return ['t', 'u', 'v'],
        '9' => return ['w', 'x', 'y', 'z'],
        _ => return null
    }
}