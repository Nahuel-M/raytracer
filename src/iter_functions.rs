pub trait AllEqual: Iterator {
    fn all_equal(&mut self) -> bool
    where
        Self::Item: Eq,
        Self: Sized,{
            let first = self.next().unwrap();
            self.all(|value| value == first)
        }

}

impl<I: Iterator> AllEqual for I {}