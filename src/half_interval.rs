use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HalfInterval {
    /// A half-open interval [lb, ub) over integers
    /// This means that [1, 2) only represents the digit 1
    lb: u32,
    range: u32,
}
impl HalfInterval {
    pub fn new(lb: u32, range: u32) -> HalfInterval {
        return HalfInterval { lb, range };
    }
    pub fn shift(&self, delta: i32) -> HalfInterval {
        return HalfInterval::new((self.lb as i32 + delta) as u32, self.range);
    }
    pub fn lb(&self) -> u32 {
        self.lb
    }
    pub fn delta(&self) -> u32 {
        self.range
    }
    pub fn intersect(&self, other: &HalfInterval) -> Option<HalfInterval> {
        // Order the
        let (left, right) = if self.lb() < other.lb() {
            (self, other)
        } else {
            (other, self)
        };

        let dlb = right.lb() - left.lb();
        let overlap = dlb < left.delta();

        if overlap {
            let left_end = right.lb();
            let new_delta = min(left.delta() - dlb, right.delta());
            return Some(HalfInterval::new(left_end, new_delta));
        } else {
            return None;
        }
    }
    pub fn diff(&self, other: &HalfInterval) -> (Option<HalfInterval>, Option<HalfInterval>) {
        // Compute the difference between self and other, i.e. self / other, read as
        // self-remove-other in set terminology

        // Determine intersection, which determines what "self remove other" looks like
        let inter = self.intersect(other);

        let retval = match inter {
            None => {
                if self.lb() < other.lb() {
                    (Some(*self), None)
                } else {
                    (None, Some(*self))
                }
            }
            Some(inter) => {
                // The intersection is exact
                if inter == *self {
                    (None, None)
                } else {
                    let inter_lb = inter.lb();
                    let left_delta = inter_lb - self.lb();
                    let left = match left_delta {
                        0 => None,
                        v => Some(HalfInterval::new(self.lb, v + 1)),
                    };

                    let remaining_da = self.delta() - left_delta;
                    let right = if remaining_da <= inter.delta() {
                        None
                    } else {
                        // NOTE: THIS LINE MAY CAUSE ISSUES, MAYBE?
                        let new_lb = inter_lb + inter.delta();
                        let new_delta = remaining_da - inter.delta();
                        Some(HalfInterval::new(new_lb, new_delta))
                    };
                    return (left, right);
                    // (left, right)
                }
            }
        };

        return retval;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let a = HalfInterval::new(1, 9);
        let b = HalfInterval::new(2, 7);

        assert_eq!(HalfInterval::new(2, 7), a.intersect(&b).unwrap());
        assert_eq!(a, a.intersect(&a).unwrap());
    }

    #[test]
    fn test_diff() {
        let a = HalfInterval::new(1, 9);
        let b = HalfInterval::new(3, 2);

        let (l, r) = a.diff(&b);

        assert_eq!(HalfInterval::new(1, 3), l.unwrap());
        assert_eq!(HalfInterval::new(5, 5), r.unwrap());

        assert_eq!((None, None), a.diff(&a));
    }
}
