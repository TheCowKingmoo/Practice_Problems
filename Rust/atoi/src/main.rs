
fn main() {
    let s: String = String::from("42");
    //let s: String = String::from("-1");
    let r = my_atoi(s);
    println!("{}", r);
}

pub fn my_atoi(str: String) -> i32 {

    if str.len() == 0  {
        return 0;
    }

    let mut negative: i32 = 1;
    let mut stack: Vec<char> = Vec::new();


    let chars: Vec<char> = str.chars().collect();
    let mut last_index = 0;

    // runs number of whitespace times
    for i in 0..chars.len()  {
        let current: u32 = chars[i] as u32;

        if current == 32  {
            // move onto next char as we ignore whitespace
            continue;
        } else if current == 45  {
            // save this value to multiply final result with
            negative = -1;
            last_index = i+1;
            break;
        }  else if current == 43  {
            last_index = i+1;
            break;
        }  else if current < 48 || current > 57 {
            // error - first real value is nota sign/digit
            return 0;
        }
        // save leaving off space for the next loop
        last_index = i;
        // kill the loop, we have our starting location
        break;
    }

    let mut non_zero_front = false;
    // runs n - whitespace times
    for i in last_index..chars.len()  {
        let current_value = chars[i] as u32;

        if(!non_zero_front)  {
            if(current_value == 48)  {
                continue;
            }  else  {
                non_zero_front = true;
            }
        }
        if current_value >= 48 && current_value <= 57  {
            stack.push(chars[i]);
        }  else  {
            break;
        }
    }


    if stack.len() > 10  {
        if negative == -1  {
            return  std::i32::MIN;
        }  else  {
            return std::i32::MAX;
        }
    }


    let result = convert_char_stack_to_int(&mut stack);

    if result >= (std::i32::MAX as u32 + 1) && negative == -1  {
        return std::i32::MIN;
    }
    if result >= std::i32::MAX as u32 && negative == 1  {
        return std::i32::MAX;
    }
    println!("hi");
    return result as i32 * negative;
}

// pops the last value in the stack to add to a longer int value
// eg: stack with 1, 2 in it would look like this
//  pop 2
//  10^0 * 2 + total sum = 1 * 2 + 0 = 2 -> store 2 into total sum
//  pop 1
//  10^1 * 1 + total sum = 10 * 1 + 2 = 12 -> store 12 into total sum

pub fn convert_char_stack_to_int(stack: &mut Vec<char>) -> u32  {
    let mut final_value: u32 = 0;

    for i in 0..stack.len()  {
        // used to get the "tens place"
        let power: u32 = u32::pow(10, i as u32);    //todo - POW is not cheap -> use some sort of barrel shifter instead
        // get the char
        let current: char = stack.pop().unwrap();
        // convert char to 0-9
        let int_value: u32 = convert_char_to_single_digit(current);
        // find actual value of the char
        if(std::u32::MAX / power) < int_value  {
            return std::u32::MAX;
        }
        let tens_place: u32 = power * int_value;
        // store
        final_value = final_value + tens_place;
    }

    return final_value;

}

pub fn convert_char_to_single_digit(input: char) -> u32  {
    let v: u32 = input as u32 - 48;
    return v;
}