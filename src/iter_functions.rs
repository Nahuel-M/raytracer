pub trait AllEqual: Iterator {
    fn all_equal(&mut self) -> bool
    where
        Self::Item: Eq,
        Self: Sized,{
            let first = self.next().unwrap();
            self.all(|value| value == first)
        }

}

// pub trait OptionMin<T: PartialOrd>: Iterator<Item=Option<T>>{
//     fn min_option(&mut self) -> T
//     where
//         Self::Item: PartialOrd,
//         Self: Sized,{
//             let first = self.next().unwrap();
//             self.all(|value| value == first)
//         }

// }

impl<I: Iterator> AllEqual for I {}