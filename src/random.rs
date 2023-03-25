use rand::{rngs::ThreadRng, Rng};

pub trait RngExtension {
    fn random_slice_index<T>(&mut self, slice: &[T]) -> Option<usize>;

    fn random_slice_entry<'slice, T>(&mut self, slice: &'slice [T]) -> Option<&'slice T> {
        self.random_slice_index(slice).map(|index| &slice[index])
    }
}

impl RngExtension for ThreadRng {
    fn random_slice_index<T>(&mut self, slice: &[T]) -> Option<usize> {
        if slice.is_empty() {
            None
        } else {
            Some(self.gen_range(0..slice.len()))
        }
    }
}
