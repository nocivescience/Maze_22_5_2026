use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;

const ALTO: usize = 21;
const ANCHO : usize= 21;

fn main() {
    let mut laberinto =&vec![vec![0; ANCHO]; ALTO];
    let laberinto2 = laberinto;
    for i in laberinto {
        println!("{:?}", i);
    }
}