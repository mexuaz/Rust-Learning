
use std::time::Instant;
use rand::Rng;
use rayon::prelude::*; //  crate for parallel processing


// @param n size of vector
fn simple(n: usize) ->Vec<f64> {
    let mut rng = rand::thread_rng();
    
    // Initialize a random vector of size n
    let mut vec: Vec<f64> = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.gen_range(0.0..1.0));
    }
    
    // Divide each element of vector by its index
    for (i, x) in vec.iter_mut().enumerate() {
        if i != 0 { 
            *x /= i as f64; 
        }
    }
    
    return vec;

}

fn with_map_red(n: usize) -> Vec<f64>{
    let mut rng = rand::thread_rng();
    
    // Initialize a random vector of size n
    let mut vec: Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..1.0)).collect();
    
    // Divide each element of vector by its index
    for (i, x) in vec.iter_mut().enumerate() {
        if i != 0 { *x /= i as f64; }
    }
    return vec;
}

fn with_concurrency(n: usize) ->Vec<f64> {
    let mut rng = rand::thread_rng();
    
    // Initialize a random vector of size n
    let mut vec: Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..1.0)).collect();
    
    // Divide each element of vector by its index in parallel
    vec.par_iter_mut().enumerate().for_each(|(i, x)| {
        if i != 0 { *x /= i as f64; }
    });

    return vec;
    
}

fn main() {
    let n = 1000000usize;

    let mut start = Instant::now();
    let vec = simple(n);
    let mut duration = start.elapsed();
    println!("Time elapsed in simple() is: {:?}", duration);
   
    start = Instant::now();
    let _vec2 = with_map_red(n);
    duration = start.elapsed();
    println!("Time elapsed in with_map_red() is: {:?}", duration);
    
    start = Instant::now();
    let _vec3 = with_concurrency(n);
    duration = start.elapsed();
    println!("Time elapsed in with_concurrency() is: {:?}", duration);

    // print fist 10 elements
    for i in &vec[0..10] {
        println!("{}", i);
    }
}