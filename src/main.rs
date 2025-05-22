use std::collections::{HashMap};
fn length_of_longest_substring(s:&str) -> i32 {
    let mut longest = 0;
    let mut start = 0;
    let mut seen = HashMap::new();
    for (i, char) in s.chars().enumerate() {
        if seen.contains_key(&char) && seen[&char] >= start {
            start = seen[&char] + 1;
        }
        seen.insert(char, i);
        longest = longest.max(i - start + 1);
    }
    longest as i32
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vecs = nums1.clone();
    vecs.append(&mut nums2.clone());
    vecs.sort_by(|a, b| b.cmp(a));
    if vecs.len() % 2 == 1 {
        vecs[vecs.len() / 2] as f64
    } else {
        (vecs[(vecs.len() / 2) - 1] + vecs[vecs.len() /2]) as f64 / 2.0
    }
}


fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if chars.len() <= 1 {
        return s;
    }
    let mut longest = &chars[0..1];
    for i in 0..n {
        let mut cl = i;
        let mut cr = i;
    //  First look at odd length
        while cl > 0 && cr + 1 < n && chars[cl-1] == chars[cr+1] {
            cl -= 1;
            cr += 1;
        }
        if cr - cl + 1 >= longest.iter().len() {
            longest = &chars[cl..=cr];
        }
    //  Next look at even length
        if i+1 < n && chars[i] == chars[i+1] {
            cl = i;
            cr = i + 1;
            while cl > 0 && cr + 1 < n && chars[cl-1] == chars[cr + 1] {
                cl -= 1;
                cr += 1;
            }
            if cr - cl + 1 >= longest.iter().len() {
                longest = &chars[cl..=cr];
            }
        }
    }
    longest.iter().collect()
}
fn main() {
    let s = "bab";
    // let s = "aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa";
    println!("{}", length_of_longest_substring(s));
    let v1:Vec<i32> = vec![1,3];
    let v2: Vec<i32> = vec![2,4];
    println!("{}", find_median_sorted_arrays(v1, v2));
    let ss = String::from(s);
    println!("{}", longest_palindrome(ss));
}
