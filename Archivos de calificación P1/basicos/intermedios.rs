fn main() {
    let mut a: i64 = 909;

    println!("=======================================================================");
    println!("==================================IF===================================");
    println!("=======================================================================");

    if (a > 50) {
        println!("IF CORRECTO");
    }else if (a == 56) {
        println!("IF INCORRECTO");
    } else {
        println!("IF INCORRECTO");
    }

    println!("");
    println!("=======================================================================");
    println!("=============================IFs ANIDADOS==============================");
    println!("=======================================================================");
    let aux: i64 = 10;
    if aux > 0 {
        println!("PRIMER IF CORRECTO");
        if true && (aux == 1) {
            println!("SEGUNDO IF INCORRECTO");
        } else if aux > 10 {
            println!("SEGUNDO IF INCORRECTO");
        } else {
            println!("SEGUNDO IF CORRECTO");
        }
    }else if aux <= 3 {
        println!("PRIMER IF INCORRECTO");
        if true && (aux == 1) {
            println!("SEGUNDO IF INCORRECTO");
        } else if aux > 10 {
            println!("SEGUNDO IF INCORRECTO");
        } else {
            println!("SEGUNDO IF CORRECTO");
        } 
    } else if aux == a {
        println!("PRIMER IF INCORRECTO");
        if true && (aux == 1) {
            println!("SEGUNDO IF INCORRECTO");
        } else if aux > 10 {
            println!("SEGUNDO IF INCORRECTO");
        } else {
            println!("SEGUNDO IF CORRECTO");
        } 
    }

    println!("");
    println!("=======================================================================");
    println!("=================================WHILE=================================");
    println!("=======================================================================");

    let mut index: i64 = 0;

    while (index >= 0) {

        if (index == 0) {
            index = index + 100;
        } else if (index > 50) {
            index = index / 2 - 25;
        } else {
            index = (index / 2) - 1;
        } 

        println!("{}",index);
    }

    println!("");
    println!("=======================================================================");
    println!("================================WHILE-2================================");
    println!("=======================================================================");

    index = -2;
    index = index + 1;

    while (index != 12) {
        index = index + 1;
        if (index == 0 || index == 1 || index == 11 || index == 12) {
            println!("*********************************************************************************************************");
        } else if (index == 2) {
            println!("**********  ***************  ******                 ******                 ******              **********");
        } else if (index >= 3 && index <= 5) {
            println!("**********  ***************  ******  *********************  *************  ******  **********************");
        } else if (index == 6) {
            println!("**********  ***************  ******                 ******                 ******  **********************");
        } else if (index >= 7 && index <= 9) {
            println!("**********  ***************  ********************   ******  *************  ******  **********************");
        } else if (index == 10) {
            println!("**********                   ******                 ******  *************  ******              **********");
        } 
    }

    println!("");
    println!("=======================================================================");
    println!("=============================TRANSFERENCIA=============================");
    println!("=======================================================================");

    a = -1;
    while (a < 5) {
        a = a + 1;
        if a == 3 {
            println!("a");
            continue;
        } else if a == 4 {
            println!("b");
            break;
        } 

        println!("El valor de a es: {}, ", a);
    }

    println!("Se debió imprimir");

    println!("");
    println!("=======================================================================");
    println!("==================================FOR==================================");
    println!("=======================================================================");

    for i in 0..9 {

        let mut output: String = "".to_string();
        for j in 0..(10 - i) {
            output = output + &" ".to_string();
        }

        for k in 0..i {
            output = output + &"* ".to_string();
        }
        println!("{}",output);

    }

    println!("");
    println!("=======================================================================");
    println!("=================================FOR-2=================================");
    println!("=======================================================================");

    let arr = [1,2,3,4,5,6];
    for i in [0,1,2,3,4,5] {
        println!("{},{},{},{},{},{}", arr[i] == 1, arr[i] == 2, arr[i] == 3, arr[i] == 4, arr[i] == 5, arr[i] == 6);
    }

    println!("");
    println!("=======================================================================");
    println!("=================================FOR-3=================================");
    println!("=======================================================================");
    for e in [0,1,2,3,4,5] {
        if (arr.len() > e) {
            println!("{},{},{},{},{},{}", e*arr[e],e*arr[e],e*arr[e],e*arr[e],e*arr[e],e*arr[e]);
        }
    }
}