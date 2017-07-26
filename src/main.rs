extern crate compare;
extern crate rand;
extern crate time;
extern crate rayon;
#[macro_use] extern crate match_all;
use rand::Rng;
use compare::{Compare, natural};
use std::cmp::Ordering::{Less, Equal, Greater};

fn i32_rand_array(len: usize) -> Vec<i32> {
    let mut res = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        res.push(rng.gen::<i32>());
    }
    res
}

fn i32_merge_sort(array: &mut [i32]) {
    if array.len() > 1 {
        //Sorting each half
        let half = array.len() / 2;
        let whole = array.len();
        let mut merged = Vec::with_capacity(whole);
        {
            let (mut lh, mut rh) = array.split_at_mut(half);
            rayon::join(|| i32_merge_sort(&mut lh), || i32_merge_sort(&mut rh));
        }
        {
            let (left_half, right_half) = array.split_at_mut(half);

            //Merging the halves together
            let mut left = 0;
            let mut right = 0;
            let cmp = natural();
            while left < left_half.len() && right < right_half.len() {
                match_all!{ cmp.compare(&left_half[left], &right_half[right]),
                    Less | Equal => {
                        merged.push(left_half[left]);
                        left += 1;
                    },
                    Greater | Equal => {
                        merged.push(right_half[right]);
                        right += 1;
                    }
                }
            }

            for i in left .. left_half.len() {
                merged.push(left_half[i]);
            }
            for i in right .. right_half.len() {
                merged.push(right_half[i]);
            }
        }
        for (i, x) in merged.iter().enumerate() {
            array[i] = *x;
        }
    }
}

fn main() {
    for _ in 0..10 {
        let mut list: Vec<i32> = i32_rand_array(1000000);
        let before = time::now();
        i32_merge_sort(&mut list);
        let after = time::now();
        println!("{}", after - before);
    }
}
