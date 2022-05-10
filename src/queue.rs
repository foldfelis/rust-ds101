#[derive(Debug)]
pub struct Queue<T> {
    es: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        return Queue { es: Vec::new() }
    }

    pub fn len(&self) -> usize {
        return self.es.len();
    }

    pub fn push(&mut self, e: T) {
        self.es.push(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() < 1 {
            return None;
        } else {
            return Some(self.es.remove(0));
        }
    }
}
