#![feature(portable_simd)]
use std::{simd::{f32x2, StdFloat, f32x8, f32x4}, time::{SystemTime, SystemTimeError}, hash::{Hasher, DefaultHasher}, io::{stdout, Write}};

fn get_hash(vec: &Vec<f32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for elem in vec.iter() {
        hasher.write_u32(elem.to_bits());
    }
    hasher.finish()
}

fn main() -> Result<(), SystemTimeError> {
    let arry = (0..100_000_000).map(|x| x as f32).collect::<Vec<f32>>();

    println!("Computing sqrt of first {} numbers", arry.len());
    
    // No simd
    {
        let mut result: Vec<f32> = Vec::with_capacity(arry.len());

        print!("Test 1: no simd");
        let start = SystemTime::now();
        for i in 0..arry.len() {
            if i % (arry.len() / 10) == 0 {
                print!(".")
            }
            result.push(arry[i].sqrt());
        }
        let dur = SystemTime::now().duration_since(start)?;
        println!(" {:.3}s", dur.as_secs_f32());

        let hash = get_hash(&result);
        println!("Result hash: {hash:X}");
    }

    // 32x2
    {
        let mut result: Vec<f32> = Vec::with_capacity(arry.len());

        print!("Test 2: f32x2");
        let start = SystemTime::now();
        for i in (0..arry.len()).step_by(2) {
            if i % (arry.len() / 10) == 0 {
                print!(".");
                stdout().flush().unwrap();
            }
            let vals = arry[i..i+2].try_into().unwrap();
            let simd_vec = f32x2::from_array(vals);
            let simd_result = simd_vec.sqrt();
            result.append(&mut simd_result.to_array().to_vec());
        }
        let dur = SystemTime::now().duration_since(start)?;
        println!(" {:.3}s", dur.as_secs_f32());

        let hash = get_hash(&result);
        println!("Result hash: {hash:X}");
    }

    // 32x4
    {
        let mut result: Vec<f32> = Vec::with_capacity(arry.len());

        print!("Test 3: f32x4");
        let start = SystemTime::now();
        for i in (0..arry.len()).step_by(4) {
            if i % (arry.len() / 10) == 0 {
                print!(".");
                stdout().flush().unwrap();
            }
            let vals = arry[i..i+4].try_into().unwrap();
            let simd_vec = f32x4::from_array(vals);
            let simd_result = simd_vec.sqrt();
            result.append(&mut simd_result.to_array().to_vec());
        }
        let dur = SystemTime::now().duration_since(start)?;
        println!(" {:.3}s", dur.as_secs_f32());

        let hash = get_hash(&result);
        println!("Result hash: {hash:X}");
    }

    // 32x8
    {
        let mut result: Vec<f32> = Vec::with_capacity(arry.len());

        print!("Test 4: f32x8");
        let start = SystemTime::now();
        for i in (0..arry.len()).step_by(8) {
            if i % (arry.len() / 10) == 0 {
                print!(".");
                stdout().flush().unwrap();
            }
            let vals = arry[i..i+8].try_into().unwrap();
            let simd_vec = f32x8::from_array(vals);
            let simd_result = simd_vec.sqrt();
            result.append(&mut simd_result.to_array().to_vec());
        }
        let dur = SystemTime::now().duration_since(start)?;
        println!(" {:.3}s", dur.as_secs_f32());

        let hash = get_hash(&result);
        println!("Hash: {hash:X}");
    }

    Ok(())
}
