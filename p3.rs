use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;

const ALTO: usize = 21;
const ANCHO: usize = 21;

const DIRECCIONES : [(isize, isize); 4] = [
    (0, -2),
    (0, 2),
    (-2, 0),
    (2, 0),
];

fn mostrar_laberinto(laberinto: &[Vec<i32>]) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..ALTO {
        for j in 0.. ANCHO {
            if laberinto[i][j]==1 {
                print!(" ");
            }else {
                print!("kk");
            }
        }
        println!();
    }
}

fn generar_laberinto(x: usize, y: usize, laberinto: &mut Vec<Vec<i32>>) {
    laberinto[y][x] = 1;
    mostrar_laberinto(laberinto);
    thread::sleep(Duration::from_millis(50));
    let mut dirs: Vec<usize> = vec![0, 1, 2, 3];
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    dirs.shuffle(&mut rng);
    for i in dirs {
        let (dx, dy) = DIRECCIONES[i];
        let nx = x as isize +dx;
        let ny: isize = y as isize +dy;
        if nx> 0 && nx<(ANCHO-1) as isize && ny > 0 && ny< (ALTO-1) as isize {
            let nx_u = nx as usize;
            let ny_u: usize = ny as usize;
            if (laberinto[ny_u][nx_u])==0 {
                let muro_x = (x as isize +dy/2) as usize;
                let muro_y = (x as isize + dx/2) as usize;
                laberinto[muro_y][muro_x] =1;
                generar_laberinto(nx_u,  ny_u, laberinto);
            }
        }
    };
}
fn main() {
    let mut laberinto = vec![vec![0; ANCHO]; ALTO];
    generar_laberinto(1, 1, &mut laberinto);
}