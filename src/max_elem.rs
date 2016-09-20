use std::cmp;

trait MaxElem<T> {
    fn max_elem<'a>(&'a self) -> Option<&'a T>;
}

impl<T> MaxElem<T> for [T] where T: Ord {
    fn max_elem<'a>(&'a self) -> Option<&'a T> {
        let len = self.len();

        match len {
            0 | 1 => self.first(),
            _ => {
                let mid = len / 2;
                let (left, right) = self.split_at(mid);

                let left = left.max_elem();
                let right = right.max_elem();

                match (left, right) {
                    (Some(a), Some(b)) => Some(cmp::max(a, b)),
                    _ => left.or(right)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::MaxElem;

    #[test]
    fn test_max_elem() {
        let array = [11, 2, 9, 1, 3, 88];
        assert_eq!(Some(&88), array.max_elem());
    }
}