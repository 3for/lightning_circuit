#![feature(rustc_private)]

extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::u8;

use self::Test::*;

enum Test {
    Valid,
    AndInsteadOfXor
}

fn main() {
    // rust implementation for https://blog.decentriq.ch/proving-hash-pre-image-zksnarks-zokrates/
    let preimage = [00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,05];
    let mut hasher = Sha256::new();
    hasher.input(&preimage);
    print!("512-bit `5` sha256:{}\n", hasher.result_str());

    println!("valid: ");
    gen(Valid);
    println!("using AND instead of XOR: ");
    gen(AndInsteadOfXor);
}

fn gen(test: Test) {
    let mut r2;
    let mut x;
    let mut h1;
    let mut h2;
    {
        let mut hash = Sha256::new();
        
        hash.input_str("SCIPR");
        r2 = std::vec::from_elem(0u8, hash.output_bytes());
        hash.result(r2.as_mut_slice());
    }
   
    
    {
        let mut hash = Sha256::new();
        hash.input_str("LAB");
        x = std::vec::from_elem(0u8, hash.output_bytes());
        hash.result(x.as_mut_slice());
    }

    
    let r1 = {
        let mut v = vec![];
        for (a, b) in r2.iter().zip(x.iter()) {
            if let AndInsteadOfXor = test {
                v.push(a & b);
            } else {
                v.push(a ^ b);
            }
        }

        v
    };

    {
        let mut hash = Sha256::new();
        hash.input(&r1);
        h1 = std::vec::from_elem(0u8, hash.output_bytes());
        hash.result(h1.as_mut_slice());
    }

    {
        let mut hash = Sha256::new();
        hash.input(&r2);
        h2 = std::vec::from_elem(0u8, hash.output_bytes());
        hash.result(h2.as_mut_slice());
    }

    print!("h1_bv = int_list_to_bits("); into_bin(&h1);
    print!("h2_bv = int_list_to_bits("); into_bin(&h2);
    print!("x_bv = int_list_to_bits("); into_bin(&x);
    print!("r1_bv = int_list_to_bits("); into_bin(&r1);
    print!("r2_bv = int_list_to_bits("); into_bin(&r2);
}

fn into_bin(a: &Vec<u8>) {
    let mut first = true;
    print!("{{");
    for a in a.iter() {
        print!("{}{}",
                {if !first { ", " } else {first = false; ""}},
                a
                );
    }
    println!("}}, 8);");
}
