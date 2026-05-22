const ALTO: usize = 21;
const ANCHO: usize = 21;

// En Rust, solemos agrupar las coordenadas en tuplas para que sea más limpio
const DIRECCIONES: [(isize, isize); 4] = [
    (0, -2), // Arriba
    (0, 2),  // Abajo
    (-2, 0), // Izquierda
    (2, 0),  // Derecha
];

fn generar_laberinto(x: usize, y: usize, laberinto: &mut Vec<Vec<i32>>) {
    laberinto[x][y] = 1;
}

fn mostrar_laberinto(laberinto: &Vec<Vec<i32>>) {
    for i in 0..ALTO {
        for j in 0..ANCHO {
            if laberinto[i][j] == 1 {
                print!(" ");
            } else {
                print!("hh");
            }
        }
        println!(); // En Rust, println!() ya añade el salto de línea al final
    }
}

fn main() {
    println!("Este es el mejor elemento...");

    // Creamos una matriz (Vector de Vectores) inicializada en 0
    let mut laberinto = vec![vec![0; ANCHO]; ALTO];

    generar_laberinto(1, 1, &mut laberinto);
    generar_laberinto(1, 2, &mut laberinto);
    generar_laberinto(1, 3, &mut laberinto);

    mostrar_laberinto(&laberinto);
}