//================================
//Library importing

extern crate rand_chacha;

use std::env;
use rand::{SeedableRng, Rng};
use rand::distributions::Standard;

//================================
//generate random vector with the same seed

fn gen_vec(seed: u64, count:usize) -> Vec<u32>{
    let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    rng.sample_iter(Standard).take(count).collect()
}

//================================
//Stuff for code Optimization class

static mut dummy: [i32;(3072 * 1024)] = [0;(3072 * 1024)];

fn clear_cache(cacheSize: usize){
    for i in 0..cacheSize{
        unsafe{
            dummy[i] += 1;
        }
    }
}

//================================
//Sorting Codes

fn quick_sort(vals: &mut Vec<u32>, start:usize, end:usize){
    //stop condition
    if (start + 1) < end{
        //iterators
        let mut pivot = start;
        let mut i = start + 1;
        let mut j = end - 1;

        //actual code
        while i <= j{
            if pivot < i{
                if vals[pivot] > vals[j] {
                    let x = vals[j];
                    vals[j] = vals[pivot];
                    vals[pivot] = x;
                    pivot = j;
                }
                j -= 1;
            }
            else {
                if vals[pivot] < vals[i] {
                    let x = vals[i];
                    vals[i] = vals[pivot];
                    vals[pivot] = x;
                    pivot = i;
                }
                i += 1;
            }
        }
        //recursive sort the rest of the vector
        quick_sort(vals, start, pivot);
        quick_sort(vals, pivot+1, end);
    }
}

fn radix_sort(vals: &mut Vec<u32>){
    vals[0];
}

//================================
//main code
fn main(){
    let arguments :Vec<_> = env::args().collect();
    if arguments.len() < 3 {
        println!("Usage:./Program [vector size] [seed]");
    }else{
        println!("arg count: {}",arguments.len());
        let cacheSize:usize = 3072 * 1024; //calculated here to be done only once
        let count = arguments[1].parse().ok().expect("please enter a number");
        let seed = arguments[2].parse().ok().expect("Please enter a number");
        let mut vals = gen_vec(seed, count);

        println!("{:?}",vals);
        quick_sort(&mut vals, 0, count);
        println!("{:?}",vals);

        clear_cache(cacheSize);

    }
}
