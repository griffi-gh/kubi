use std::{thread::JoinHandle, collections::VecDeque};

pub struct KubiPool<T, R> {
    callback: fn(T) -> R,
    threads: Vec<JoinHandle<()>>,
}

struct Task<T> {
    priority: u8,
    data: T,
}

fn task_loop<T, R>() {
    let tasks = VecDeque::<Task<T>>::new();
    loop {

    };
}

impl<T: 'static, R: 'static> KubiPool<T, R> {
    pub fn new(threads: usize, callback: fn(T) -> R) -> Self {
        Self {
            callback,
            threads: (0..threads).map(|_| {
                std::thread::spawn(move || task_loop::<T, R>())
            }).collect(),
        }
    }

    pub fn resize(&mut self, threads: usize) {

    }
}
