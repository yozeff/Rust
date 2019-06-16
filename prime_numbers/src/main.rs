//Joseph Harrison 2019
//factorise a number into its primes
use std::io;

fn is_prime(n: i64) -> bool {
    //accounts for 1, 0 and negative integers
    if n < 2 {
        return false;
    //two is prime despite being even
    } else if n == 2 {
        return true;
    //any even numbers are composite
    } else if n % 2 == 0 {
        return false;
    } else {
        //potential factor
        let mut i = 3 as i64;
        //square root of i
        let r = (n as f64).sqrt() as i64;
        //only iterate factors up to √n 
        while i < r {
            //if i divides n, n is composite
            if n % i == 0 {
                return false;
            } else {
                //only test odd factors
                i = i + 2;
            }
        }
        //if all factors tested n is prime
        return true;
    }
}

fn prime_factorise(mut n: i64) -> Vec<i64> {
    //vector to store primes
    let mut factors: Vec<i64> = Vec::new();
    while n != 1 {
        //divisible by 2 test can
        //be done seperately to use
        //only odd factors later
        if n % 2 == 0 {
            n = n / 2;
            factors.push(2);
        } else {
            let mut i = 3;
            //test odd factors
            while n % i != 0 || !is_prime(i) {
                i = i + 2;
            }
            n = n / i;
            factors.push(i);
        }
    }
    return factors;
}

fn main() {

    println!("number:");

    let mut n = String::new();

    //get number input
    io::stdin().read_line(&mut n)
        .expect("failed read");

    //parse and raise errors
    let n: i64 = n.trim().parse()
        .expect("failed parse");

    if is_prime(n) {
        println!("{} is prime", n);
    } else {
        println!("{} is composite", n);
    }

    println!("prime factors:");

    if n != 0 {

        if n < 0 {
            //handle negatives
            let mut factors = prime_factorise(n * -1);
            factors.push(-1);
            
            for i in &factors {
                println!("{}", i);
            }
        } else {
            let factors = prime_factorise(n);
            
            for i in &factors {
                println!("{}", i);
            }
        }

    } else {
        println!("0 has no prime factorisation");
    }
}

