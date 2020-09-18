/*
From Leetcode
Problem::
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order

My Solution::
  Simply add the chars to a stack and take off the last char in the stack when we encounter a rhs char
 */

/*
 Potential Fixes
  - better system for constant values -> OCP, easier to read, less code
  - mapping for lhs and rhs values -> OCP, easier to read, less code
  - less lazy variable naming
  - switch from passing the nested return false to rusts use of errors
*/

fn main() {
    let s: String = String::from("(]");
    println!("{}", is_valid(s));
}

fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut err_check: bool = true; // used to pass error from helper functions
    for c in chars {
        match c {
            '{' => push_to_stack(c, &mut stack),
            '}' => err_check = pop_from_stack(c, &mut stack),
            '[' => push_to_stack(c, &mut stack),
            ']' => err_check = pop_from_stack(c, &mut stack),
            '(' => push_to_stack(c, &mut stack),
            ')' => err_check = pop_from_stack(c, &mut stack),
            _ => return false,
        };

        if !err_check {
            return false;
        }
    }

    return stack.len() == 0;    // ensures that we will only give back true if nothing is left in stack
}

/*
 Just in case I want to do something else in the future
*/
fn push_to_stack(c: char, stack: &mut Vec<char>) {
    stack.push(c);
}

/*
 pops the last value in the stack and compares to passed char. if they are not mapped return false
 if mapped return trues

 expectation is that only valid rhs inputs as c will be given
*/
fn pop_from_stack(c: char, stack: &mut Vec<char>) -> bool {
    if stack.len() == 0 {
        return false;
    }

    let popped: char = stack.pop().unwrap();

    if (c == '}' && popped != '{') || (c == ']' && popped != '[') || (c == ')' && popped != '(') {
        return false;
    }

    return true;
}
