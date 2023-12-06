use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HalfInterval {
    /// A half-open interval [lb, ub) over integers
    /// This means that [1, 2) only represents the digit 1
    lb: u32,
    ub: u32,
}
impl HalfInterval {
    pub fn new(lb: u32, ub: u32) -> HalfInterval {
        return HalfInterval { lb, ub };
    }
    pub fn from_lb_length(lb: u32, l: u32) -> HalfInterval {
        return HalfInterval::new(lb, lb + l);
    }
    pub fn shift(&self, delta: i32) -> HalfInterval {
        return HalfInterval::new(
            (self.lb as i32 + delta) as u32,
            (self.ub as i32 + delta) as u32,
        );
    }
    pub fn lb(&self) -> u32 {
        self.lb
    }
    pub fn ub(&self) -> u32 {
        self.ub
    }
    pub fn delta(&self) -> u32 {
        self.ub - self.lb
    }
    pub fn intersect(&self, other: &HalfInterval) -> Option<HalfInterval> {
        if self.lb < other.ub && other.lb < self.ub {
            return Some(HalfInterval::new(
                max(self.lb, other.lb),
                min(self.ub, other.ub),
            ));
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
            Some(remove) => {
                if remove == *self {
                    (None, None)
                } else {
                    let lb_inter = remove.lb;
                    let ub_inter = remove.ub;

                    let left = if lb_inter != self.lb {
                        Some(HalfInterval::new(self.lb, lb_inter))
                    } else {
                        None
                    };
                    let right = if ub_inter != self.ub {
                        Some(HalfInterval::new(ub_inter, self.ub))
                    } else {
                        None
                    };

                    (left, right)
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
        let a = HalfInterval::new(1, 10);
        let b = HalfInterval::new(2, 9);

        assert_eq!(HalfInterval::new(2, 9), a.intersect(&b).unwrap());
        assert_eq!(a, a.intersect(&a).unwrap());
    }

    #[test]
    fn test_diff() {
        let a = HalfInterval::new(1, 10);
        let b = HalfInterval::new(3, 5);

        let (l, r) = a.diff(&b);

        assert_eq!(HalfInterval::new(1, 3), l.unwrap());
        assert_eq!(HalfInterval::new(5, 10), r.unwrap());

        assert_eq!((None, None), a.diff(&a));
    }
}
