//Joseph Harrison 2019
//bubble sort
use std::io;
mod quick_sort;

fn bubble_sort(mut xs: Vec<i32>) -> Vec<i32> {
    //tracks if swaps have been performed
    let mut swaps: bool = true;
    let mut round: u32 = 1;
    
    while swaps == true {
        
        println!("round {}", round);
        
        //at the beginning of each iteration 
        //this hasn't happened
        swaps = false;
        
        for i in 1..xs.len() {
         
            for y in xs.iter() {
                
                print!("{} ", y);
            }
            
            print!("\n");
            for _ in 0..i - 1 {
                
                print!("  ");
            }
            println!("^ ^");
            
            if xs[i] < xs[i - 1] {
                
                //a swap has occured now
                swaps = true;
                let x = xs[i];
                xs[i] = xs[i - 1];
                xs[i - 1] = x;
            }
        }
        round = round + 1;
    }
    println!("no swaps this round");
    xs
}

fn main() {
    
    //number of items to be sorted
    //initially number is a string in order to get input
    let mut number = String::new();
    println!("number of items: ");
    io::stdin().read_line(&mut number).expect("failed read number");
    
    //attempt parse
    let number: u16 = number.trim().parse().expect("failed number parse");
    
    //initialise vector with user values
    let mut xs: Vec<i32> = Vec::new();
    for i in 0..number {
        
        //get item
        let mut x = String::new();
        println!("item {}: ", i + 1);
        io::stdin().read_line(&mut x).expect("failed read item");    
        let x: i32 = x.trim().parse().expect("failed item parse");
        
        xs.push(x);
    }
    
    println!("\nquick_sort call");
    let ys: Vec<i32> = quick_sort::quick_sort(xs.clone());
    println!("\nbubble_sort call");
    let zs: Vec<i32> = bubble_sort(xs.clone());
    
    //output results
    println!("sorted: ");
    println!("i :xs :bs :qs");
    for i in 0..xs.len() {

        println!("{} : {} : {} : {}", i + 1, xs[i], ys[i], zs[i]);
    }
}
