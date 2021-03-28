use std::io;

fn main() {
   sucesion();
}


fn sucesion(){
    let mut y: i32 = 0;
    let mut x: i32 = 1;
    let mut v: i32 = 0;
    let mut contador=0;
    let mut max_number = String::new();
    
    println!("write the maximum size");
    io::stdin().read_line(&mut max_number).expect("failed");
 
    let max_number: i32 = max_number.trim().parse().expect("failed convert");
    println!("{}",v);
     while contador<max_number {  
     v=x+y; //1 - 2 - 3 - 5
     y=x; // 1 - 1 - 2 - 
     x=v; // 1 - 2 - 3 -
     println!("{}",y); //1 - 2 - 3 - 
     contador+=1;
     }
}
