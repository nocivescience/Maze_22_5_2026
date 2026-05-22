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
}

fn mostrar_laberinto(laberinto: &Vec<Vec<i32>>) {
    for i in 0..ALTO {
        for j in 0..ANCHO {
            if laberinto[i][j] ==1 {
                print!(" ");
            }else{
                print!("hh");
            }
        }
        println!();
    }
}

fn main() {
    println!("Este es el mejor programa hecho con Rust");
    let mut laberinto = vec![vec![0; ANCHO]; ALTO];
    for fila in laberinto {
        println!("{:?}", fila)
    };
    generar_laberinto(1,1, &mut laberinto);
    generar_laberinto(1,2, &mut laberinto);
    generar_laberinto(1,3, &mut laberinto);
    
}