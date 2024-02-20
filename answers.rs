fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn first_occurrence_of_number(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let limit = (num as f64).sqrt() as i32;
    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].clone();
    for s in &strings[1..] {
        prefix.truncate(prefix.chars().zip(s.chars()).take_while(|&(a, b)| a == b).count());
    }
    prefix
}

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).copied()
}

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
    }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];
    for &num in &arr[1..] {
        max_ending_here = max_ending_here.max(0) + num;
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}

fn main() {
    // Testing the functions
    println!("{}", is_palindrome("racecar")); // true
    println!("{}", is_palindrome("hello")); // false

    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", first_occurrence_of_number(&arr, 3)); // Some(2)

    let words = "hello world";
    println!("{:?}", shortest_word(words)); // Some("hello")

    println!("{}", is_prime(7)); // true
    println!("{}", is_prime(10)); // false

    let nums = [1, 2, 3, 4, 5];
    println!("{}", median(&nums)); // 3

    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("{}", longest_common_prefix(&strings)); // fl

    println!("{:?}", kth_smallest_element(&arr, 3)); // Some(3)

    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
        right: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
    }));
    println!("{}", max_depth(root)); // 2

    println!("{}", reverse_string("hello")); // olleh

    let arr1 = [1, 3, 5];
    let arr2 = [2, 4, 6];
    println!("{:?}", merge_sorted_arrays(&arr1, &arr2)); // [1, 2, 3, 4, 5, 6]

    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{}", max_subarray_sum(&nums)); // 6
}
