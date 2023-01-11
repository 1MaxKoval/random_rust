use std::cmp::{ Eq };

pub trait Storage<T: Eq> {
    fn store(&mut self, val: T);
    fn get(&mut self) -> &T;
    fn show(&self) -> Vec<&T>;
}

pub trait Flippable<T: Eq> {
    fn flip(&mut self) -> Vec<&T>;
}

impl<T: Storage<T> + Eq> Flippable<T> for T {

    fn flip(&mut self) -> Vec<&T> {
        return self.show().reverse();
    }

}

struct MessageStore<T: Eq> {
    values: Vec<T>,
    position: usize
}

fn new<T: Eq>(values: Vec<T>, position: usize) -> Result<MessageStore<T>, String> {
    if position < 0 || position >= values.len() {
        return Err("Position cannot be outside of the ordered structure!".to_string());
    }
    return Ok(MessageStore {
        values: values,
        position: position
    })
}

impl<T: Eq> Storage<T> for MessageStore<T> {
        
    fn store(&mut self, msg: T) {
        self.values.push(msg);
    }

    fn get(&mut self) -> &T {
        let r  = &self.values[self.position];
        self.position = (self.position + 1) % self.values.len();
        return r;
    }

    fn show(&self) -> Vec<&T> {
        let mut r = Vec::new();
        for i in &self.values {
            r.push(i);
        }
        return r;
    }

}

fn main() {}
