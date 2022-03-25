fn relacionalesYlogicos (){
    
    let a:i64 = 100;
    let b:i64 = 100;
    let c:i64 = 7;
    let f:bool = true;
    let j:f64 = 10.0;
    let k:f64 = 10.0;
    
    println!("");
    println!("");
    if a > b || b < c {
        println!(">>>>>> Esto no debería de imprimirse");
    }else{
        println!(">>>>>> Esto debería de imprimirse")
    }
    
    
    if a == b && j == k || 14 != c {
        println!(">>>>>> Esto debería de imprimirse")
    }else{
        println!(">>>>>> Esto no debería de imprimirse");
    }
    
    let val1:i64 = 5;
    let resp:i64 = 5;
    let mut valorVerdadero : i64 = 100;
    
    if((valorVerdadero == (50 + 50 + (val1 - val1))) && ! ! ! ! ! ! ! ! ! ! true) {
        println!(">>>>>> En este lugar deberia de entrar :)");
        valorVerdadero = 50;
    }
    else if (f || (valorVerdadero > 50)) && ((resp != 100) && ! ! ! ! ! f){
        println!(">>>>>> Aca no deberia de entrar :ccc");
        valorVerdadero = 70;
    }
    else{
        println!(">>>>>> Aca no deberia de entrar :cccc");
    }

    
    
} 

fn numeroPar( x:i64)-> bool{
    
    let mut ingreso : bool = false;
    
    if x % 2 == 0 {
        println!(">>>>> numeroPar ingreso a if verdadero, {} es par",x);
        ingreso = true;
    }
    else {
        println!(">>>>> numeroPar ingreso a if falso, {} no es par",x);
        ingreso = false;
    }
    
    return ingreso;
}

fn sumatoria( a:i64, b:i64, c:i64)-> i64{
    println!("La sumatoria de los valores es: {}",(a+b+c));
    return a + b +c ;
}

fn inicio(){
    let a: i64 = 5;
    let b: i64 = 10;
    let c: i64 = 3;
    let d: f64 = 8.0;
    
    let bo: bool = false;
    let bol2: bool = !bo;
    let cad: String = "imprimir".to_string();
    let cad2: String = "cadena valida".to_string();
    let letra: char = 'c';
    
    
    let val1 = 7 - (5 + 10 * (2 + 4 * (5 + 2 * 3)) - 8 * 3 * 3) + 50 * (6 * 2);
    let val2 = (2 * 2 * 2 * 2) - 9 - (8 - 6 + (3 * 3 - 6 * 5 - 7 - (9 + 7 * 7 * 7) + 10) - 5) + 8 - (6 - 5 * (2 * 3));
    let val3 = val1 + ((2 + val2 * 3) + 1 - ((2 * 2 * 2) - 2) * 2) - 2;
    
    println!("El valor de val1 es:              {}",val1);
    println!("El valor de val2 es:              {}",val2);
    println!("El valor de val3 es:              {}",val3);
    println!("El resultado de la operación es:  {}",val3);
    println!("El valor de bo es:                {}",bo);
    println!("El valor de cad es:               {}",cad);
    println!("El valor de cad2 es:               {}",cad2);
    println!("El valor de letra es:             {}",letra);
    println!("El valor de letra bol2:            {}",bol2);
    println!("");
    
    // COMENTAR
    //a = val1; // ERROR, a no es mutable
    
    let sumatoria = sumatoria(a,b,c);
    println!("El valor de sumatoria es:         {}",sumatoria);
    
    let mut cont:i64 = 0;
    loop {
    
        numeroPar(cont);
        cont = cont + 1;
        if cont == 15 {
            break;
        }
    };
    
}


fn main() {
    
    println!("----------------------");
    println!("----ARCHIVO BASICO----");
    println!("----------------------");
    
    
    inicio();
    relacionalesYlogicos();

    let abs1:i64 = 7-11;
    let abs2:f64 = 10.0;
    let raiz1:i64 = 9;
    let raiz2:f64 = 100.0;
    
    let b = [1, 2, 3, 4];
    
    let copia = b.clone();
    
    println!("");
    println!("*************PRUEBA DE NATIVAS");
    println!(" valor de b: {:?}",b);
    println!(" valor de copia: {:?}",copia);
    
    println!(" valor absoluto1: {}",abs1.abs());
    println!(" valor absoluto2: {}",abs2.abs());
    println!(" valor raiz1: {}",(raiz1 as f64).sqrt());
    println!(" valor raiz2: {}",raiz2.sqrt());
    
}

/*
----------------------
----ARCHIVO BASICO----
----------------------
El valor de val1 es:              214
El valor de val2 es:              412
El valor de val3 es:              1439
El resultado de la operación es:  1439
El valor de bo es:                false
El valor de cad es:               imprimir
El valor de cad2 es:               cadena valida
El valor de letra es:             c
El valor de letra bol2:            true

La sumatoria de los valores es: 18
El valor de sumatoria es:         18
>>>>> numeroPar ingreso a if verdadero, 0 es par
>>>>> numeroPar ingreso a if falso, 1 no es par
>>>>> numeroPar ingreso a if verdadero, 2 es par
>>>>> numeroPar ingreso a if falso, 3 no es par
>>>>> numeroPar ingreso a if verdadero, 4 es par
>>>>> numeroPar ingreso a if falso, 5 no es par
>>>>> numeroPar ingreso a if verdadero, 6 es par
>>>>> numeroPar ingreso a if falso, 7 no es par
>>>>> numeroPar ingreso a if verdadero, 8 es par
>>>>> numeroPar ingreso a if falso, 9 no es par
>>>>> numeroPar ingreso a if verdadero, 10 es par
>>>>> numeroPar ingreso a if falso, 11 no es par
>>>>> numeroPar ingreso a if verdadero, 12 es par
>>>>> numeroPar ingreso a if falso, 13 no es par
>>>>> numeroPar ingreso a if verdadero, 14 es par


>>>>>> Esto debería de imprimirse
>>>>>> Esto debería de imprimirse
>>>>>> En este lugar deberia de entrar :)

*************PRUEBA DE NATIVAS
 valor de b: [1, 2, 3, 4]
 valor de copia: [1, 2, 3, 4]
 valor absoluto1: 4
 valor absoluto2: 10
 valor raiz1: 3
 valor raiz2: 10
*/