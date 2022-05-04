// PILA
fn pila_vacia(vec: &mut Vec<i64>) -> bool {
    vec.len() == 0
}

fn apilar(capacidad: usize, vec: &mut Vec<i64>, value: i64) {
    if vec.len() < capacidad {
        vec.insert(vec.len(), value);
    } else {
        println!("La pila ha llegado a su maxima capacidad");
    }
}

fn desapilar(vec: &mut Vec<i64>) -> i64 {
    if !pila_vacia(vec) {
        return vec.remove(vec.len()-1);
    } else {
        println!("La pila no tiene elementos");
    }
    0
}

// COLA
fn cola_vacia(vec: &mut Vec<i64>) -> bool {
    vec.len() == 0
}

fn encolar(capacidad: usize, vec: &mut Vec<i64>, value: i64) {
    if vec.len() < capacidad {
        vec.push(value);
    } else {
        println!("La cola ha llegado a su maxima capacidad");
    }
}

fn desencolar(vec: &mut Vec<i64>) -> i64 {
    if !cola_vacia(vec) {
        return vec.remove(0);
    } else {
        println!("La cola no tiene elementos");
    }
    0
}

fn main() {
    let capacidad: usize = 10;
    let mut pila: Vec<i64> = Vec::with_capacity(capacidad - 2);
    let mut cola: Vec<i64> = vec![1,2,3,4,5];

    let datos: [i64; 5] = [10,20,30,40,50];

    for dato in datos {
        apilar(capacidad, &mut pila, dato);
    }
    
    println!("{:?}", pila);
    println!("{}", desapilar(&mut pila));
    apilar(capacidad, &mut pila, 1250);
    apilar(capacidad, &mut pila, 2200);
    apilar(capacidad, &mut pila, 3500);
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{}", desapilar(&mut pila));
    println!("{:?}", pila);
    println!("Capacidad de pila");
    println!("{}", pila.capacity());
    println!("");

    encolar(capacidad, &mut cola, 800);
    println!("{:?}", cola);
    println!("{}", desencolar(&mut cola));
    encolar(capacidad, &mut cola, 1250);
    encolar(capacidad, &mut cola, 2200);
    encolar(capacidad, &mut cola, 3500);
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{}", desencolar(&mut cola));
    println!("{:?}", cola);
    println!("Capacidad de cola");
    println!("{}", cola.capacity());
    println!("");

    // vectores entre vectores
    let mut lista: Vec<Vec<i64>> = Vec::new();
    lista.push(vec![0; 10]);
    lista.push(vec![1; 10]);
    lista.push(vec![2; 10]);
    lista.push(vec![3; 10]);
    lista.push(vec![75,23,10,29,30,12,49,10,93]);
    println!("{:?}", lista);
    println!("");
    println!("{:?}", lista[0]);
    println!("{:?}", lista[1]);
    println!("{:?}", lista[2]);
    println!("{:?}", lista[3]);
    println!("{:?}", lista[4]);
    println!("{}", lista[4][8]);
    println!("");

    let vec = vec!["Hola", "!", "Sale", "Este", "Semestre", "2022"];
    println!("{}", vec.contains(&"Semestre") || vec.contains(&"2023"));
    println!("{}", vec.contains(&"Semestre") && vec.contains(&"2023"));
    println!("{}", vec.contains(&"Hola"));
}
