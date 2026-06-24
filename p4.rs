use colored::*;

const ALTO: usize = 21;
const ANCHO: usize = 21;

const DIRECCIONES: [(isize, isize); 4] = [
    (0, -2),
    (0, 2),
    (-2, 0),
    (2, 0),
];
fn generar_laberinto(x: usize, y: usize, laberinto: & mut Vec<Vec<i32>>) {
    laberinto[x][y] = 1;
}

fn mostrar_laberinto(laberinto: &Vec<Vec<i32>>) {
    for i in 0..ALTO {
        for j in 0..ANCHO {
            if laberinto[i][j] ==1 {
                print!("r");
            }else{
                print!("@");
            }
        };
        println!();
    }
}

fn main() {
    println!("Este es un elemento");
    let mut laberinto = vec![vec![0; ANCHO]; ALTO];
    generar_laberinto(1, 1, &mut laberinto);
    generar_laberinto(1, 1, &mut laberinto);
    generar_laberinto(1, 1, &mut laberinto);

    mostrar_laberinto(&laberinto);
}