use std::{iter, ops::Add};

pub struct TupleIter<T> {
    n: usize,
    from: T,
    to: T,
    current: Vec<T>,
}

impl<T> TupleIter<T> {
    pub fn new(n: usize, from: T, to: T) -> TupleIter<T> {
        assert!(n > 0);
        TupleIter {
            n,
            from,
            to,
            current: vec![],
        }
    }
}

impl<T: Eq + Add<Output = T> + Copy + From<u8>> Iterator for TupleIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.len() == 0 {
            self.current = iter::repeat(self.from).take(self.n).collect();
        } else {
            let mut j = self.current.len() as isize - 1;
            while j >= 0 && self.current[j as usize] == self.to {
                j -= 1;
            }

            if j == -1 {
                return None;
            }
            let new_value = self.current[j as usize] + T::from(1);
            self.current[j as usize..].fill(new_value);
        }

        Some(self.current.clone())
    }
}
