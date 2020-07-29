use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell<> value
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: no one else can concurrently modify self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references (because we never give any out)
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: only this thread can mutate (because !Sync)
        unsafe { *self.value.get() }
    }
}

// below code won't compile because of compiler safety checks

// #[cfg(test)]
// mod test {
    // use super::Cell;

    // #[test]
    // fn bad() {
    //     use std::sync::Arc;
    //     let x = Arc::new(Cell::new(42));
    //     let x1 = Arc::clone(&x);
    //     std::thread::spawn(move || {
    //         x.set(43);
    //     });
    //     let x2 = Arc::clone(&x);
    //     std::thread::spawn(move || {
    //         x2.set(44);
    //     });
    // }

    // #[test]
    // fn bad2() {
    //     let x = Cell::new(4);
    //     let first = &x.get();
    //     x.set(3);
    //     eprintln!("{}", first);
    // }
// }
