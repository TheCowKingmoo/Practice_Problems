/*
  Write Up
  Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

  Solution: use two pointers. look at every string start at index 0 - 1, 0 - 2, ... 0 - n, then move onto every string at 1 - 2, 1 - 3,... 1-n
  so that we look at every possible string. 

  best case: O(n^2)
    improve -> start at longest possible string at cut once we find a string. as we start from highest we know there is not a higher len str
    will changes to best case: O(n) and worst == O(n^2)
  worst case: O(n^2)


*/


fn main() {
    let s: String = String::from("racecar");
    
    println!("{}",longest_palindrome(s) );
}

pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut largest_length = 0;
    let mut largest_string = String::from("");

    /*
      i tracks the starting location for the slice
      j tracks the ending location for the slice

      recall -> slicing is exlusive on the end
    */

    for i in 0..chars.len()  {
        for j in (i+1)..chars.len()+1  {
            let current_slice: &[char] = &chars[i..j];
            let palin_result = is_palin(current_slice, i, j);
            if palin_result && ( current_slice.len() > largest_length )  {
                largest_length = current_slice.len();
                largest_string = current_slice.into_iter().collect();
            }
        }
    }
    return largest_string;
}

pub fn is_palin(chars: &[char], i: usize, j:usize) -> bool  {
    let mut start_left = 0;
    let mut start_right = chars.len() - 1;

    for i in 0..chars.len()/2  {
        if chars[start_left] != chars[start_right]  {
            return false;
        }
        start_left = start_left + 1;
        start_right = start_right - 1;
    }

    return true;

}
