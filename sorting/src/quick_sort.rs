//Joseph Harrison 2019

pub fn quick_sort(mut xs: Vec<i32>) -> Vec<i32> {
    
    if xs.len() > 1 {
    
        //select pivot element
        let x = xs[0];
        
        println!("\npivot is {}", x);
    
        //subvectors store elements less than
        //the pivot
        let mut less: Vec<i32> = Vec::new();
        let mut more: Vec<i32> = Vec::new();
        
        //push elements into subvectors based of ordering with x
        for i in 1..xs.len() {
            
            if xs[i] < x {
                
                less.push(xs[i]);
                println!("pushing {} to less", xs[i]);
            }
            //equal elements to x will be placed in more
            else {
                
                more.push(xs[i]);
                println!("pushing {} to more", xs[i]);
            }
        }
        
        //sort two subvectors
        println!("\ncall on less");
        let less = quick_sort(less);
        println!("\ncall on more");
        let more = quick_sort(more);
        
        //copy less elements first
        println!("less:");
        for (i, &y) in less.iter().enumerate() {
            
            println!("{} : {}", i + 1, y);
            xs[i] = y;
        }
        
        //copy pivot element
        xs[less.len()] = x;
        
        //copy more element
        println!("more:");
        for (i, &y) in more.iter().enumerate() {
            
            println!("{} : {}", i + 1, y);
            xs[i + less.len() + 1] = y;
        }
        } else {
            
            println!("subvector contains 0 or 1 element(s) - edge case");
        }
    
    xs
}