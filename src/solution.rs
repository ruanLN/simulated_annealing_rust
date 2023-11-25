#[derive(Clone, Debug)]
pub struct Solution {
    items: Vec<u32>,
    box_size: u32,
    boxes: u32,
    fitness: f64,
}

impl Solution {
    pub fn new(bs: u32, items: Vec<u32>) -> Self {
        Self {
            box_size: bs,
            boxes: Self::boxes(&items, bs),
            fitness: Self::eval(&items, bs),
            items,
        }
    }

    pub const fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn print(self) {
        println!(
            "\nnumero de caixas da sol: {}, fitness: {}",
            self.boxes, self.fitness
        );
        println!("Distribuição: {:?}", self.items);
    }

    pub fn copy_mutate(&self) -> Self {
        let i = rand::random::<usize>() % self.items.len();
        let j = (0..)
            .map(|_| rand::random::<usize>() % self.items.len())
            .find(|j| j != &i)
            .unwrap();
        let mut vec = self.items.clone();

        vec.swap(i, j);
        Self::new(self.box_size, vec)
    }

    pub fn eval(items: &Vec<u32>, box_size: u32) -> f64 {
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

    pub fn boxes(items: &Vec<u32>, box_size: u32) -> u32 {
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
