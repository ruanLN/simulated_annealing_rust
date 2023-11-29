mod solution;

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

use solution::BinPackingProblemSolution;

use crate::solution::execute_simulated_annealing;

fn main() -> Result<()> {
    let path = Path::new("test_files/N1C1W1_A.BPP");

    // Open the path in read-only mode, returns io::Result<File>`
    let f = File::open(path)?;

    let file = BufReader::new(&f);

    let items = file
        .lines()
        .skip(1)
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    if let Some((b, items)) = items.split_first() {
        //começa a marcar o tempo
        let now = Instant::now();

        let (sol, s_best) = execute_simulated_annealing(BinPackingProblemSolution::new(*b, items));
        let sec = now.elapsed().as_secs();
        let ms = now.elapsed().subsec_millis();
        //fim da marcaçao de tempo
        sol.print();
        println!("Solução final encontrada em {sec}.{ms} segundos");
        s_best.print();
    }

    Ok(())
}
