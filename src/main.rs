extern crate rand;

mod solution;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use crate::solution::Solution;

fn main() -> Result<()> {
    let path = Path::new("test_files/N1C1W1_A.BPP");

    // Open the path in read-only mode, returns io::Result<File>`
    let f = File::open(path)?;

    let file = BufReader::new(&f);

    let mut data = file
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .filter_map(|s| s.parse::<u32>().ok());

    let b: u32 = data.next().unwrap();
    let items: Vec<u32> = data.collect();

    //começa a marcar o tempo
    let now = Instant::now();

    let sol = Rc::new(Solution::new(b, items));
    let mut s_best = sol.clone();
    let mut s = sol.clone();

    let mut s_linha;
    let mut t: f64 = 0.8;
    let mut auxcounter: i32 = 0;
    while t > 0.001 {
        for _ in 0..2000 {
            s_linha = Rc::new(s.copy_mutate());
            if s_linha.fitness() > s_best.fitness() {
                s_best = s_linha.clone();
            }
            let delta: f64 = s_linha.fitness() - s.fitness();
            if delta >= 0f64 || rand::random::<f64>() < (delta / t).exp() {
                auxcounter = 0;
                s = s_linha.clone();
            } else {
                auxcounter += 1;
                if auxcounter == 10 && t < 0.01f64 {
                    //Condição de parada caso haja muitas iterações sem troca de solução em um sistema relativamente esfriado
                    t = 0f64;
                    break;
                }
            }
        }
        t *= 0.99;
    }
    let sec = now.elapsed().as_secs();
    let ms = now.elapsed().subsec_millis();
    //fim da marcaçao de tempo
    (*sol).clone().print();
    println!("Solução final encontrada em {sec}.{ms} segundos");
    (*s_best).clone().print();

    Ok(())
}
