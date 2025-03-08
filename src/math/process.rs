use std::iter::zip;
use std::slice::Iter;

pub struct Process {
    times: Vec<f64>,
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

impl Process {
    pub fn new() -> Self {
        Process {
            times: vec![0.],
            x: vec![0.],
        }
    }

    pub fn iter(&self) -> ProcessIter {
        ProcessIter {
            iter: zip(self.times.iter(), self.x.iter()),
        }
    }
}

pub fn process_build() {
    let b = Process::new();
    println!("{:?}", b.times);
    println!("{:?}", b.x);

    b.iter().for_each(|(t, x)| println!("{t}{x}"));
}
