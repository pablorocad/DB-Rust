
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


fn hanoi(n: i32, desde: i32, hacia: i32, via: i32) {
    if n > 0 {
        hanoi(n - 1, desde, via, hacia);
        println!("Mover disco de palo {} a palo {}", desde, hacia);
        hanoi(n - 1, via, hacia, desde);
    }
}
 

fn main(){
    println!("--------------------------");
    println!("----ARCHIVO RECURSIVOS----");
    println!("--------------------------");
    
    
    println!("Fibonacci de 6 {}",fibonacci(6));;
    
    println!("");
    println!("Hanoi");
	hanoi(3, 1 ,2 ,3);
}


/*
--------------------------
----ARCHIVO RECURSIVOS----
--------------------------
Fibonacci de 6 13

Hanoi
Mover disco de palo 1 a palo 2
Mover disco de palo 1 a palo 3
Mover disco de palo 2 a palo 3
Mover disco de palo 1 a palo 2
Mover disco de palo 3 a palo 1
Mover disco de palo 3 a palo 2
Mover disco de palo 1 a palo 2
*/