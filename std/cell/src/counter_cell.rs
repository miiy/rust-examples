use std::cell::Cell;

pub struct CellCounter {
    value: Cell<i32>,
}

impl CellCounter {
    fn new() -> Self {
        Self { value: Cell::new(0) }
    }

    fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }

    fn get(&self) -> i32 {
        self.value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_counter() {
        let counter = CellCounter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        assert_eq!(counter.get(), 1);
        counter.increment();
        assert_eq!(counter.get(), 2);
    }
}