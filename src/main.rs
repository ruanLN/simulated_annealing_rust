extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Solution {
    items: Vec<u32>,
    box_size: u32,
    boxes: u32,
    fitness: f64,
}

impl Solution {
    fn new(bs: u32, items: Vec<u32>) -> Self {
        Self {
            box_size: bs,
            boxes: Self::boxes(&items, bs),
            fitness: Self::eval(&items, bs),
            items,
        }
    }

    fn print(self) {
        println!(
            "\n numero de caixas da sol: {}, fitness: {}",
            self.boxes, self.fitness
        );
        println!("Distribuição: {:?}", self.items);
    }

    fn copy_mutate(&self) -> Self {
        let i = rand::random::<usize>() % self.items.len();
        let mut j = i;
        while i == j {
            j = rand::random::<usize>() % self.items.len();
        }
        let mut vec = self.items.clone();
        vec.swap(i, j);
        Self::new(self.box_size, vec)
    }

    fn eval(items: &Vec<u32>, box_size: u32) -> f64 {
        let mut temp: u32 = 0;
        let mut fit_temp: f64 = 0_f64;
        for item in items {
            temp += *item;
            if temp > box_size {
                fit_temp += (f64::from(temp - item)) / f64::from(box_size).powi(2);
                temp = *item;
            }
        }

        fit_temp += f64::from(temp).powi(2);
        fit_temp /= f64::from(Self::boxes(items, box_size));

        fit_temp
    }

    fn boxes(items: &Vec<u32>, box_size: u32) -> u32 {
        let mut current_box = 0;
        let mut total_boxes = 0;
        for item in items {
            if current_box > box_size {
                current_box = 0;
                total_boxes += 1;
            }
            current_box += item;
        }
        if current_box > 0 {
            total_boxes += 1;
        }
        total_boxes
    }
}

fn main() {
    let path = Path::new("test_files/N1C1W1_A.BPP");

    // Open the path in read-only mode, returns io::Result<File>`
    let f = match File::open(path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(f) => f,
    };

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
            if s_linha.fitness > s_best.fitness {
                s_best = s_linha.clone();
            }
            let delta: f64 = s_linha.fitness - s.fitness;
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
}
