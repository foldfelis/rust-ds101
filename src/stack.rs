#[derive(Debug)]
pub struct Stack<T> {
    es: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        return Stack { es: Vec::new() }
    }

    pub fn len(&self) -> usize {
        return self.es.len();
    }

    pub fn push(&mut self, e: T) {
        self.es.push(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.es.pop();
    }
}
