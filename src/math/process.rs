use std::iter::zip;
use std::slice::Iter;

pub struct Process {
    t: Vec<f64>,
    x: Vec<f64>,
}

pub struct ProcessIter<'a> {
    iter: std::iter::Zip<Iter<'a, f64>, Iter<'a, f64>>,
}

impl<'a> Iterator for ProcessIter<'a> {
    type Item = (&'a f64, &'a f64);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait IterableProcess {
    fn t(&self) -> &Vec<f64>;
    fn x(&self) -> &Vec<f64>;
    fn iter(&self) -> ProcessIter {
        ProcessIter {
            iter: zip(self.t().iter(), self.x().iter()),
        }
    }
}

impl Process {
    pub fn new() -> Self {
        Process {
            t: vec![0.],
            x: vec![0.],
        }
    }
}

impl IterableProcess for Process {
    #[inline(always)]
    fn t(&self) -> &Vec<f64> {
        &self.t
    }
    #[inline(always)]
    fn x(&self) -> &Vec<f64> {
        &self.x
    }
}

pub fn process_build() {
    let b = Process::new();
    println!("{:?}", b.t);
    println!("{:?}", b.x);

    b.iter().for_each(|(t, x)| println!("{t}{x}"));
}
