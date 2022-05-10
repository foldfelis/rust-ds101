#[derive(Debug)]
pub struct Stack10 {
    es: [i64; 10],
    top: isize
}

impl Stack10 {
    pub fn new() -> Stack10 {
        return Stack10 { es: [0; 10], top: -1 }
    }

    pub fn len(&self) -> isize {
        return self.top + 1;
    }

    pub fn push(&mut self, e: i64) {
        self.increase_top();
        self.es[self.top as usize] = e;
    }

    pub fn pop(&mut self) -> i64 {
        let e = self.es[self.top as usize];
        self.decrease_top();

        return e;
    }

    fn increase_top(&mut self) {
        self.top += 1;
        assert!(self.top <= 10);
    }

    fn decrease_top(&mut self) {
        self.top -= 1;
        assert!(self.top >= -1);
    }
}
