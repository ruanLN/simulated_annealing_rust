use std::rc::Rc;

pub trait Solution {
    fn fitness(&self) -> f64;
    fn copy_mutate(&self) -> Self;
}

#[derive(Clone, Debug)]
pub struct BinPackingProblemSolution {
    items: Vec<u32>,
    box_size: u32,
    boxes: u32,
    fitness: f64,
}
impl BinPackingProblemSolution {
    pub fn new(bs: u32, items: &[u32]) -> Self {
        Self {
            box_size: bs,
            boxes: Self::boxes(items, bs),
            fitness: Self::eval(items, bs),
            items: items.to_vec(),
        }
    }

    pub fn print(self) {
        println!(
            "\nnumero de caixas da sol: {}, fitness: {}",
            self.boxes, self.fitness
        );
        println!("Distribuição: {:?}", self.items);
    }

    pub fn eval(items: &[u32], box_size: u32) -> f64 {
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

    pub fn boxes(items: &[u32], box_size: u32) -> u32 {
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
impl Solution for BinPackingProblemSolution {
    fn fitness(&self) -> f64 {
        self.fitness
    }

    fn copy_mutate(&self) -> Self {
        let rng = &mut rand::thread_rng();
        let a = rand::seq::index::sample(rng, self.items.len(), 2);
        let (i, j) = (a.index(0), a.index(1));
        let mut vec = self.items.clone();

        vec.swap(i, j);
        Self::new(self.box_size, &vec)
    }
}

pub fn execute_simulated_annealing<T: Solution + Clone>(instance: T) -> (T, T) {
    let sol = Rc::new(instance);
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
    (sol.as_ref().clone(), s_best.as_ref().clone())
}
