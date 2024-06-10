#[test]
fn test_bp() {
    use bplustree::BPlusTree;
    let mut bpm: BPlusTree<Info, u64> = BPlusTree::new();
}
use rust_decimal::Decimal;
struct Info {
    _price: Decimal,
    _amount: Decimal,
}
impl Clone for Info {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        return self._price == other._price && self._amount == other._amount;
    }
}
impl Eq for Info {}
impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            return Some(std::cmp::Ordering::Less);
        } else if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        return Some(std::cmp::Ordering::Greater);
    }
    fn lt(&self, other: &Self) -> bool {
        if self._price < other._price {
            return true;
        } else if self._price == other._price && self._amount <= other._amount {
            return true;
        }
        return false;
    }
    fn le(&self, other: &Self) -> bool {
        return self.lt(other) || self.eq(other);
    }
    fn ge(&self, other: &Self) -> bool {
        return self.gt(other) || self.eq(other);
    }
    fn gt(&self, other: &Self) -> bool {
        return !self.le(other);
    }
}
impl Ord for Info {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self < other {
            return std::cmp::Ordering::Less;
        } else if self == other {
            return std::cmp::Ordering::Equal;
        }
        return std::cmp::Ordering::Greater;
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self > other {
            return self.clone();
        }
        return other.clone();
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self < other {
            return self.clone();
        }
        return other.clone();
    }
}
