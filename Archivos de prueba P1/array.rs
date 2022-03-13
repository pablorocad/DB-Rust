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

/* salida */
/*
Imprimir grupos
["Grupo A", "Grupo B", "Grupo C", "Grupo D", "Grupo E"]
Imprimir equipos
[["Manchester City", "PSG", "RB Leipzig", "Club Brujas"], ["Liberpool", "Atletico de Madrid", "Porto", "Milan"], ["Ajax", "Sporting Lisboa", "Dortmund", "Besiktas"], ["Real Madrid", "Inter", "Sheriff", "Shaktar"], ["Bayern", "Benfica", "Barcelona", "Dinamo"]]
Imprimir equipos
["Manchester City", "PSG", "RB Leipzig", "Club Brujas"]
["Liberpool", "Atletico de Madrid", "Porto", "Milan"]
["Ajax", "Sporting Lisboa", "Dortmund", "Besiktas"]
["Real Madrid", "Inter", "Sheriff", "Shaktar"]
["Bayern", "Benfica", "Barcelona", "Dinamo"]

Imprimir Equipos por grupo

Grupo A -> Manchester City
Grupo A -> PSG
Grupo A -> RB Leipzig
Grupo A -> Club Brujas

Grupo B -> Liberpool
Grupo B -> Atletico de Madrid
Grupo B -> Porto
Grupo B -> Milan

Grupo C -> Ajax
Grupo C -> Sporting Lisboa
Grupo C -> Dortmund
Grupo C -> Besiktas

Grupo D -> Real Madrid
Grupo D -> Inter
Grupo D -> Sheriff
Grupo D -> Shaktar

Grupo E -> Bayern
Grupo E -> Benfica
Grupo E -> Barcelona
Grupo E -> Dinamo


Simulador de partidos

Grupo A -> Equipo1: Manchester City 2 vs Equipo2: PSG 1
Grupo A -> Equipo1: Manchester City 2 vs Equipo2: RB Leipzig 4
Grupo A -> Equipo1: Manchester City 2 vs Equipo2: Club Brujas 2
Grupo A -> Equipo1: PSG 5 vs Equipo2: RB Leipzig 4
Grupo A -> Equipo1: PSG 5 vs Equipo2: Club Brujas 2
Grupo A -> Equipo1: RB Leipzig 6 vs Equipo2: Club Brujas 2

Grupo B -> Equipo1: Liberpool 2 vs Equipo2: Atletico de Madrid 1
Grupo B -> Equipo1: Liberpool 2 vs Equipo2: Porto 4
Grupo B -> Equipo1: Liberpool 2 vs Equipo2: Milan 2
Grupo B -> Equipo1: Atletico de Madrid 5 vs Equipo2: Porto 4
Grupo B -> Equipo1: Atletico de Madrid 5 vs Equipo2: Milan 2
Grupo B -> Equipo1: Porto 6 vs Equipo2: Milan 2

Grupo C -> Equipo1: Ajax 2 vs Equipo2: Sporting Lisboa 1
Grupo C -> Equipo1: Ajax 2 vs Equipo2: Dortmund 4
Grupo C -> Equipo1: Ajax 2 vs Equipo2: Besiktas 2
Grupo C -> Equipo1: Sporting Lisboa 5 vs Equipo2: Dortmund 4
Grupo C -> Equipo1: Sporting Lisboa 5 vs Equipo2: Besiktas 2
Grupo C -> Equipo1: Dortmund 6 vs Equipo2: Besiktas 2

Grupo D -> Equipo1: Real Madrid 2 vs Equipo2: Inter 1
Grupo D -> Equipo1: Real Madrid 2 vs Equipo2: Sheriff 4
Grupo D -> Equipo1: Real Madrid 2 vs Equipo2: Shaktar 2
Grupo D -> Equipo1: Inter 5 vs Equipo2: Sheriff 4
Grupo D -> Equipo1: Inter 5 vs Equipo2: Shaktar 2
Grupo D -> Equipo1: Sheriff 6 vs Equipo2: Shaktar 2

Grupo E -> Equipo1: Bayern 2 vs Equipo2: Benfica 1
Grupo E -> Equipo1: Bayern 2 vs Equipo2: Barcelona 4
Grupo E -> Equipo1: Bayern 2 vs Equipo2: Dinamo 2
Grupo E -> Equipo1: Benfica 5 vs Equipo2: Barcelona 4
Grupo E -> Equipo1: Benfica 5 vs Equipo2: Dinamo 2
Grupo E -> Equipo1: Barcelona 6 vs Equipo2: Dinamo 2


Imprimir estadisticas realeas

Grupo A -> Manchester City
Partidos jugados
6
Partidos ganados
4
Partidos empatados
0
Partidos perdidos
2

Grupo A -> PSG
Partidos jugados
6
Partidos ganados
3
Partidos empatados
2
Partidos perdidos
1

Grupo A -> RB Leipzig
Partidos jugados
6
Partidos ganados
2
Partidos empatados
1
Partidos perdidos
3

Grupo A -> Club Brujas
Partidos jugados
6
Partidos ganados
1
Partidos empatados
1
Partidos perdidos
4

Grupo B -> Liberpool
Partidos jugados
6
Partidos ganados
6
Partidos empatados
0
Partidos perdidos
0

Grupo B -> Atletico de Madrid
Partidos jugados
6
Partidos ganados
2
Partidos empatados
1
Partidos perdidos
3

Grupo B -> Porto
Partidos jugados
6
Partidos ganados
1
Partidos empatados
2
Partidos perdidos
3

Grupo B -> Milan
Partidos jugados
6
Partidos ganados
1
Partidos empatados
1
Partidos perdidos
4

Grupo C -> Ajax
Partidos jugados
6
Partidos ganados
6
Partidos empatados
0
Partidos perdidos
0

Grupo C -> Sporting Lisboa
Partidos jugados
6
Partidos ganados
3
Partidos empatados
0
Partidos perdidos
3

Grupo C -> Dortmund
Partidos jugados
6
Partidos ganados
3
Partidos empatados
0
Partidos perdidos
3

Grupo C -> Besiktas
Partidos jugados
6
Partidos ganados
0
Partidos empatados
0
Partidos perdidos
6

Grupo D -> Real Madrid
Partidos jugados
6
Partidos ganados
5
Partidos empatados
0
Partidos perdidos
1

Grupo D -> Inter
Partidos jugados
6
Partidos ganados
3
Partidos empatados
1
Partidos perdidos
2

Grupo D -> Sheriff
Partidos jugados
6
Partidos ganados
2
Partidos empatados
1
Partidos perdidos
3

Grupo D -> Shaktar
Partidos jugados
6
Partidos ganados
0
Partidos empatados
2
Partidos perdidos
4

Grupo E -> Bayern
Partidos jugados
6
Partidos ganados
6
Partidos empatados
0
Partidos perdidos
0

Grupo E -> Benfica
Partidos jugados
6
Partidos ganados
2
Partidos empatados
2
Partidos perdidos
2

Grupo E -> Barcelona
Partidos jugados
6
Partidos ganados
2
Partidos empatados
1
Partidos perdidos
3

Grupo E -> Dinamo
Partidos jugados
6
Partidos ganados
0
Partidos empatados
1
Partidos perdidos
5
*/