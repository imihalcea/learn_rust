#![feature(termination_trait_lib)]
#![feature(test)]

extern crate test;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Termination;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_10_000_optimized(bencher:&mut Bencher) -> impl Termination{

    let file = File::open("benches/10_000.txt").expect("can't open data set file");
    let reader = BufReader::new(file);
    let mut data_set = vec![];

    for (n,line) in reader.lines().enumerate(){
        let val_str = line.expect("failed to read data set line");
        let input = i32::from_str(&val_str).expect("failed to parse str to i32");
        data_set.push(input);
    }

    let k = data_set[0]+data_set[9999];

    bencher.iter(||two_sum::optimized(data_set.clone(),k))
}


#[bench]
fn bench_10_000_brute_force(bencher:&mut Bencher) -> impl Termination{

    let file = File::open("benches/10_000.txt").expect("can't open data set file");
    let reader = BufReader::new(file);
    let mut data_set = vec![];

    for (n,line) in reader.lines().enumerate(){
        let val_str = line.expect("failed to read data set line");
        let input = i32::from_str(&val_str).expect("failed to parse str to i32");
        data_set.push(input);
    }

    let k = data_set[0]+data_set[9999];

    bencher.iter(||two_sum::brute_force(data_set.clone(),k))
}