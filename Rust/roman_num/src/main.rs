/*
Problem: https://leetcode.com/problems/roman-to-integer/
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9. 
    X can be placed before L (50) and C (100) to make 40 and 90. 
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
*/


/*
  Brute Force First Thoughts
  1. simply look at next char as adding -> if we encounter I, X, C -> subtract from next value
  2. need a way to map what each chars value is. enum, constants? Only 6 instances for subtract, hasn't changed in like a thousand years so constants would probably be fine.
  3. single convert function that holds a match with all values





  result
  Runtime: 4 ms, faster than 60.19% of Rust online submissions for Roman to Integer.
    Memory Usage: 2 MB, less than 91.67% of Rust online submissions for Roman to Integer.
    
    
 Potential Fixes
 change the logic in the main loop to prevent branching
 start from back of loop
 loop from i..chars.len()-1 to elimate an if statement

*/




fn main() {
    let input = String::from("MCMXCIV");
    let result = roman_to_int(input);
    println!("{}", result);
}

// we started with i32 but i feel like u32 would be a better value here
fn roman_to_int(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect(); //convert string into char array
    let mut previous_sub: bool = false;         // lets use know that we need to subtract the previous value
    let mut previous_value: i32 = 0;            // holds the previous value. this can be easily calc'd but its simplier this way
    let mut total = 0;

    for i in 0..chars.len()  {
        // easier to read vars
        let current_char = chars[i];

        // grab value
        let mut current_value = convert_single_char_to_int(current_char);

        // check if the previous value needs to be subtracted
        if previous_sub  {
            current_value = current_value - previous_value;
            previous_sub = false;
            previous_value = 0;
        // checks if we need to set next value to be subtracted
        }  else if i < chars.len() - 1  {
            let next_char = chars[i+1];
            previous_sub = is_sub(current_char, next_char);
            previous_value = current_value;
        }

        //println!("{}", total);

        if !previous_sub  {
            total = total + current_value;
        }

    }

    return total;
    
}

/*
  Simply maps roman numerals to their i23 values

  Note that this is not returning a result & no catch all on match due to guraenteed inputs
*/
fn convert_single_char_to_int(c: char) -> i32  {
    let mut value = 0;
    match c  {
        'I' => value = 1,
        'V' => value = 5,
        'X' => value = 10,
        'L' => value = 50,
        'C' => value = 100,
        'D' => value = 500,
        'M' => value = 1000,
        _ => value = 0
    }
    return value;
}

/*
 * Checks if the two values put in are one of the 6 subtraction rules
 * not using a result as there is no real way for this to give an err
 * 
 * usage of this function would be as follows for eg: IV
 * I would be the minuend as it is what will subtracted
 * V is the subtrahend -> 5-1 = 4
 * 
 * 
*/

fn is_sub(minuend: char, subtrahend: char) -> bool  {
    match minuend  {
        'I' => match subtrahend  {
            'V' => return true,
            'X' => return true,
            _ => return false
        },
        'X' => match subtrahend  {
            'L' => return true,
            'C' => return true,
            _ => return false
        },
        'C' => match subtrahend  {
            'D' => return true,
            'M' => return true,
            _ => return false
        },
        _ => return false
    }
}