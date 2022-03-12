fn mostrar_equipo_por_grupo(grupos: &mut[&str; 5], equipos: &mut[[&str; 4]; 5]) {
    for i in 0..grupos.len() {
        let grupo: String = grupos[i].to_string();
        for equipo in equipos[i] {
            let cadena: String = grupo.clone().to_string() + " -> " + equipo;
            println!("{}", cadena);
        }
        println!("");
    }
}

fn simular_partido(grupo: &str, grupos: &mut [&str; 5], equipos: &mut [[&str; 4]; 5]) {
    let goles_local: [&str; 4] = ["2","5","6","2"];
    let goles_visit: [&str; 4] = ["0","1","4","2"];

    for i in 0..grupos.len() {
        let g: String = grupos[i].to_string();
        if g == grupo {
            for j in 0..equipos[i].len() {
                let mut k: usize = j + 1;
                while k < equipos[i].len() {
                    let mut cadena: String = grupo.clone().to_string() + " -> Equipo1: " + equipos[i][j] + " ";
                    cadena = cadena + goles_local[j] + " vs Equipo2: " + equipos[i][k] + " ";
                    cadena = cadena + goles_visit[k];
                    println!("{}", cadena);
                    k = k + 1;
                }
            }
        }
    }
}

fn simulador(grupos: &mut [&str; 5], equipos: &mut [[&str; 4]; 5]) {
    let mut index: usize = 0;
    simular_partido(grupos[index], grupos, equipos);
    println!("");
    index = index + 1;
    simular_partido(grupos[index], grupos, equipos);
    println!("");
    index = index + 1;
    simular_partido(grupos[index], grupos, equipos);
    println!("");
    index = index + 1;
    simular_partido(grupos[index], grupos, equipos);
    println!("");
    index = index + 1;
    simular_partido(grupos[index], grupos, equipos);
    println!("");
}

fn main () {
    let mut grupos: [&str; 5] = ["Grupo A", "Grupo B", "Grupo C", "Grupo D", "Grupo E"];
    let mut equipos: [[&str; 4]; 5] = [
        ["Manchester City", "PSG", "RB Leipzig", "Club Brujas"],
        ["Liberpool", "Atletico de Madrid", "Porto", "Milan"],
        ["Ajax", "Sporting Lisboa", "Dortmund", "Besiktas"],
        ["Real Madrid", "Inter", "Sheriff", "Shaktar"],
        ["Bayern", "Benfica", "Barcelona", "Dinamo"],
    ];

    let tabla: [[[i64;4]; 4]; 5] = [
        [[6,4,0,2],[6,3,2,1],[6,2,1,3],[6,1,1,4]],
        [[6,6,0,0],[6,2,1,3],[6,1,2,3],[6,1,1,4]],
        [[6,6,0,0],[6,3,0,3],[6,3,0,3],[6,0,0,6]],
        [[6,5,0,1],[6,3,1,2],[6,2,1,3],[6,0,2,4]],
        [[6,6,0,0],[6,2,2,2],[6,2,1,3],[6,0,1,5]]
    ];

    println!("Imprimir grupos");
    println!("{:?}", grupos);

    println!("Imprimir equipos");
    println!("{:?}", equipos);

    println!("Imprimir equipos");
    for equipo in equipos {
        println!("{:?}", equipo);
    }

    println!("\nImprimir Equipos por grupo\n");
    mostrar_equipo_por_grupo(&mut grupos, &mut equipos);

    println!("\nSimulador de partidos\n");
    simulador(&mut grupos, &mut equipos);

    println!("\nImprimir estadisticas realeas\n");
    for i in 0..tabla.len() {
        for j in 0..tabla[i].len() {
            let cad: String = grupos[i].to_string() + " -> " + equipos[i][j];
            println!("{}", cad);

            for k in 0..tabla[i][j].len() {
                if k == 0 {
                    println!("Partidos jugados");
                } else if k == 1 {
                    println!("Partidos ganados");
                } else if k == 2 {
                    println!("Partidos empatados");
                } else {
                    println!("Partidos perdidos");
                }
                println!("{}", tabla[i][j][k]);
            }

            println!("");
        }
    }
}
