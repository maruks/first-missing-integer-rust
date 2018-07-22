/*
https://www.dailycodingproblem.com/

Given an array of integers, find the first missing positive integer in linear time and constant space.
In other words, find the lowest positive integer that does not exist in the array.
The array can contain duplicates and negative numbers as well.

For example, the input [3, 4, -1, 1] should give 2.
The input [1, 2, 0] should give 3.

You can modify the input array in-place.
*/

use std::io;

fn smallest_positive_integer_not_in_array(mut v: Vec<i32>) -> i32 {
    let mut index = 0;

    while index < v.len() {

        if v[index] == 1 + index as i32 || v[index] < 1 || v[index] > v.len() as i32 {
            index = index + 1;
        } else if v[index] > 0 && v[index] <= v.len() as i32 {

            let mut n = v[index];

            while n > 0 && n <= v.len() as i32 && v[n as usize - 1] != n as i32 {

                let nextn = v[n as usize - 1];
                v[n as usize - 1] = n;
                n = nextn;
            }

            index = index + 1;
        }
    }

    let mut result = v.len() + 1;
    index = 0;
    while index < v.len() {

        if v[index] == 1 + index as i32 {
            index = index + 1;
        } else {
            result = index + 1;
            break;
        }
    }

    result as i32
}

fn main() {
    let mut input = String::new();
    let _ok = io::stdin().read_line(&mut input);

    let mut v = Vec::new();

    for s in input.split(",") {
        let i = s.trim().parse::<i32>().unwrap();
        v.push(i);
    }

    let result = smallest_positive_integer_not_in_array(v);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use smallest_positive_integer_not_in_array;
    #[test]
    fn smallest_positive_integer_not_in_array_test() {
        assert_eq!(smallest_positive_integer_not_in_array(vec![3, 4, -1, 1]), 2);
        assert_eq!(smallest_positive_integer_not_in_array(vec![1, 2, 0]), 3);
    }
}
