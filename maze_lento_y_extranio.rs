use std::time::Duration;
use std::thread;
use rand::seq::SliceRandom;
use colored::*;

const ALTO : usize = 21;
const ANCHO : usize = 21;

const DIRECCIONES : [(isize, isize); 4] = [
    (0, -2),
    (0, 2),
    (-2, 0),
    (2, 0),
];

fn generar_laberinto(x: usize, y: usize, laberinto: &mut Vec<Vec<i32>>){
    laberinto[x][y]= 1;
    mostrar_laberinto(laberinto);
    thread::sleep(Duration::from_millis(50));
    let mut dirs = vec![0,1,2,3];
    let mut rng = rand::thread_rng();
    dirs.shuffle(&mut rng);
    for i in dirs {
        let (dx, dy) = DIRECCIONES[i];
        let nx = x as isize +dx;
        let ny = y as isize +dy;
        if nx > 0 && nx< (ANCHO-1) as isize && ny > 0 && ny < (ALTO-1) as isize {
            let nx_u = nx as usize;
            let ny_u = ny as usize;
            if laberinto[ny_u][nx_u] == 0{
                let muro_x = (x as isize +dx/2) as usize;
                let muro_y = (y as isize + dy/2) as usize;
                laberinto[muro_y][muro_x]=1;
                generar_laberinto(nx_u, ny_u, laberinto);
            }
        }
    }
}

fn mostrar_laberinto(laberinto: &Vec<Vec<i32>>) {
    for i in 0..ALTO {
        for j in 0..ANCHO {
            if laberinto[i][j] ==1 {
                print!(" ");
            }else{
                print!("██");
            }
        }
        println!();
    }
}

fn main() {
    println!("Este es el mejor programa hecho con Rust");
    let mut laberinto = vec![vec![0; ANCHO]; ALTO];
    
    generar_laberinto(1,1, &mut laberinto);
    generar_laberinto(1,2, &mut laberinto);
    generar_laberinto(1,3, &mut laberinto);

    mostrar_laberinto(&laberinto);
}