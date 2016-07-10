extern crate rand;
extern crate getopts;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::time::Instant;
use getopts::Options;
use std::env;

#[derive(Clone)]
pub struct Solution {
	items : Vec<u32>,
	box_size : u32,
	boxes: u32,
	fitness: f64,
}

impl Solution {
	fn new(bs: u32, items: Vec<u32>) -> Solution {
        let mut tmp = Solution {
            box_size: bs,
            items: items,
            boxes: 0,
			fitness: 0_f64,
        };
		tmp.eval();
		return tmp;
    }

	fn change_randomically(&mut self) {
		let i = rand::random::<usize>() % self.items.len();
		let mut j = i;
		while i == j {
			j = rand::random::<usize>() % self.items.len();
		}
		self.items.swap(i, j);
		self.eval();
	}

	fn eval(&mut self) -> u32 {
		self.boxes = 0;
		let mut temp: u32 = 0;
		let mut fit_temp: f64 = 0_f64;
		for item in &self.items {
			temp += *item;
			if temp > self.box_size {
				fit_temp += (((temp - item) as f64) / self.box_size as f64).powi(2);
				temp = *item;
				self.boxes += 1;
			}
		}
		if temp != 0 {
			self.boxes += 1;
		}

		fit_temp += (temp as f64).powi(2);
		fit_temp /= self.boxes as f64;
		self.fitness = fit_temp;

		return self.boxes;
	}
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("o", "", "set output file name TODO", "NAME");
	opts.reqopt("i", "", "set input file name", "NAME");
	opts.optopt("t", "", "set start temperature to the system", "TEMP");
	opts.optopt("l", "", "set the number os iterations for temperature", "ITER");
    opts.optopt("s", "", "set the base random seed for the system TODO", "SEED");
	opts.optflag("d", "", "dinamically calculates the start temperature TODO");
	opts.optflag("r", "", "generate a simplified report output TODO");
    opts.optflag("h", "help", "print this text menu");

	let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => {
			print_usage(&program, opts);
			return;
		 }
    };

	if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

	let start_temp : f64 = matches.opt_str("t").unwrap_or(String::from("0.8")).parse::<f64>().unwrap_or(0.8f64);

	let iters : i32 = matches.opt_str("l").unwrap_or(String::from("2000")).parse::<i32>().unwrap_or(2000);

	let in_file_name = match matches.opt_str("i") {
		Some(s) => { s }
		None => {
			print_usage(&program, opts);
			return;
		}
	};

	let path = Path::new(&in_file_name);
    let display = path.display();

    // Open the path in read-only mode, returns io::Result<File>`
    let f = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,	why.description()),
        Ok(f) => f,
    };

	let file = BufReader::new(&f);
    let mut contador: u32 = 0;

	let mut b: u32 = 0;
	let mut items : Vec<u32> = Vec::new();
	for line in file.lines() {
		let trimmed = line.unwrap();
		if contador == 0 {

		} else if contador == 1 {
			b = match trimmed.parse::<u32>() {
				Ok(b) => b,
				Err(..) => 0
			};
		} else {
			let item : u32;
			item = match trimmed.parse::<u32>() {
				Ok(item) => item,
				Err(..) => 0
			};
			if item != 0 {
				items.push(item);
			}
		}
		contador += 1;
	}


	//começa a marcar o tempo
	let now = Instant::now();

	let sol  = Solution::new(b, items.clone());
	let mut s_best = Solution::new(b, items.clone());
	let mut s  = Solution::new(b, items.clone());

	let mut s_linha;
	let mut t : f64 = start_temp;
	let mut auxcounter: i32 = 0;
	while t > 0.001 {
		for _ in 0..iters {
			s_linha = s.clone();
			s_linha.change_randomically();
			if s_linha.fitness > s_best.fitness {
				s_best = s_linha.clone();
			}
			let delta: f64 = s_linha.fitness - s.fitness;
			if delta >= 0f64 {
				auxcounter = 0;
				s = s_linha.clone();
			} else if rand::random::<f64>() < (delta / t).exp() {
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
	let ms = now.elapsed().subsec_nanos()/1000000u32;
	//fim da marcaçao de tempo
	println!("\n numero de caixas da sol inicial: {}, fitness: {}", sol.boxes, sol.fitness);
	println!("Distribuição: {:?}", sol.items);
	println!("Solução final encontrada em {}.{} segundos",sec, ms);
	println!("numero de caixas da sol final: {}, fitness: {}", s_best.boxes, s_best.fitness);
	println!("Distribuição: {:?}", s_best.items);

}
