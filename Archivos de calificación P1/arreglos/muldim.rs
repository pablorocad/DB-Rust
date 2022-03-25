/* imprime los puntos por curso de cada estudiante */
fn imprimir_punteo(ests: &mut [String; 4], cursos: &mut [&str; 3], notas: &mut [[[i64;5];3];4]) {
    let ponderacion: [&str; 5] = ["1er parcial", "2do parcial", "3er parcial", "Lab", "Examen final"];
    println!("{:?}", ponderacion);
    println!("");
    for i in 0..ests.len() {
        println!("Estudinte: ");
        println!("{}", ests[i]);
        for j in 0..cursos.len() {
            let cad: String = "Curso: ".to_string() + cursos[j];
            println!("{}", cad);
            println!("{:?}", notas[i][j]);
        }
        println!("");
    }
}

/* imprime la nota total por curso de cada estudiante*/
/* parciales 43, lab sobre 32 y examen final 25*/
fn imprimir_nota_final(ests: &mut [String; 4], cursos: &mut [&str; 3], notas: &mut [[[i64;5];3];4]) {
    for i in 0..ests.len() {
        for j in 0..cursos.len() {
            let mut nota_final: f64 = 0.0;
            for k in 0..notas[i][j].len() {
                let nota: f64 = 
                    if k == notas[i][j].len() - 1 {
                        let efinal: f64 = 25.0 / 100.0;
                        (notas[i][j][k] as f64) * efinal
                    } else if k == notas[i][j].len() - 2 {
                        let lab: f64 = 32.0 / 100.0;
                        (notas[i][j][k] as f64) * lab
                    } else {
                        let parcial: f64 = (43.0 / 3.0) / 100.0;
                        (notas[i][j][k] as f64) * parcial
                    };
                nota_final = nota_final + nota;
            }

            let cad: String = ests[i].clone().to_string() + " -> Curso: " + cursos[j];
            println!("{}", cad);
            println!("{}", nota_final);
        }
        println!("");
    }
}

/* correccion de notas */
fn corregir_notas(corregir: &mut [[i64;5];3], notas: &mut [[[i64;5];3];4]) {
    for i in 0..notas.len() {
        for j in 0..notas[i].len() {
            for k in 0..notas[i][j].len() {
                notas[i][j][k] = corregir[j][k];
            }
        }
    }
}

fn main() {
    let mut estudiantes: [String; 4] = ["Lorenza".to_string(), "Rosendo".to_string(), "Fermina".to_string(), "Markel".to_string()];
    
    let mut cursos: [&str; 3] = ["Arqui1", "Archivos", "Compi2"];

    let mut notas_buenas: [[i64;5];3] = [
        [53,88,95,89,75],
        [81,51,57,67,93],
        [94,74,58,84,100]
    ];

    let mut notas: [[[i64;5];3];4] = [
        [[37,49,61,29,44],[56,60,51,68,70],[47,15,39,17,74]],
        [[69,74,52,34,36],[24,44,50,18,76],[74,60,32,63,78]],
        [[78,14,23,52,33],[28,79,77,55,24],[23,79,47,62,44]],
        [[73,53,11,49,52],[29,16,65,34,12],[72,69,30,44,37]]
    ];

    println!("\nImprimir notas por curso y estudiantes\n");
    imprimir_punteo(&mut estudiantes, &mut cursos, &mut notas);
    println!("\nImprimir nota final por curso y estudiantes\n");
    imprimir_nota_final(&mut estudiantes, &mut cursos, &mut notas);
    println!("\nMejorar notas de todos los cursos\n");
    corregir_notas(&mut notas_buenas, &mut notas);
    println!("\nImprimir nota final actualizada por curso y estudiantes\n");
    imprimir_nota_final(&mut estudiantes, &mut cursos, &mut notas);
}
