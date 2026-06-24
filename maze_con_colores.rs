use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;
use colored::*;

const ALTO: usize = 21;
const ANCHO : usize= 21;

fn main() {
    let mut laberinto = Vec::new();
    for i in 0..ALTO {
        let mut fila  = Vec::new();
        for j in 0..ANCHO {
            fila.push(j+i);
        }
        laberinto.push(fila);
    };
    for fila in laberinto {
        for i in fila {
            if i <4 {
                print!("{} ", format!("{:02}", i).red());
            }else {
                print!("{:02} ", i);
            }
        }
        println!()
    }
}