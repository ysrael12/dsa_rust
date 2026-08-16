use crate::utils::ordering::{is_ascending, is_descending, swap};

#[derive(PartialEq, Clone, Debug)]
pub enum Pivot {
    First,
    Last,
    Middle,
}
#[derive(PartialEq, Clone, Debug)]
pub enum Order {
    Ascending,
    Descending,

}

#[derive(PartialEq, Clone, Debug)]
pub enum Strategy {
    Lomuto,
    Hoare,
}

pub struct QuickSort<T> {
    pub vec: Vec<T>,
    pub pivot: Pivot,
    pub order: Order,
    pub strategy: Strategy,
}


impl <T: Copy + PartialOrd>  QuickSort<T> {
    pub fn new(vec: Vec<T>, pivot: Pivot, order: Order, strategy: Strategy) -> Self {
        QuickSort {
            vec,
            pivot,
            order,
            strategy,
        }
    }

    pub fn sort(&mut self){
        if self.vec.len() > 1 {
            let hi = self.vec.len() - 1;
            match self.strategy {
                Strategy::Lomuto => self.lomuto_sort(0, hi),
                Strategy::Hoare => self.hoare_sort(0, hi),
            }
        }
    }

    fn pick_pivot(&self, lo: usize, hi: usize) -> usize {
        match self.pivot {
            Pivot::First => lo,
            Pivot::Last => hi,
            Pivot::Middle => lo + (hi - lo) / 2,
        }
    }

    fn lomuto_sort(&mut self, lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        let p = self.lomuto_partition(lo, hi);
        if p > lo {
            self.lomuto_sort(lo, p - 1);
        }
        if p < hi {
            self.lomuto_sort(p + 1, hi);
        }
    }

    pub fn lomuto_partition(&mut self, lo: usize, hi: usize) -> usize {
        let pivot_index = self.pick_pivot(lo, hi);
        swap(pivot_index, hi, &mut self.vec);
        self.lomuto_loop(lo, hi)
    }

    pub fn lomuto_loop(&mut self, lo: usize, hi: usize) -> usize {
        let mut i = lo;
        for j in lo..hi {
            let in_position = match self.order {
                Order::Ascending => is_ascending(j, hi, &self.vec),
                Order::Descending => is_descending(j, hi, &self.vec),
            };
            if in_position {
                swap(i, j, &mut self.vec);
                i += 1;
            }
        }
        swap(i, hi, &mut self.vec);
        i
    }

    fn hoare_sort(&mut self, lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        let p = self.hoare_partition(lo, hi);
        if p > lo {
            self.hoare_sort(lo, p);
        }
        if p + 1 <= hi {
            self.hoare_sort(p + 1, hi);
        }
    }

    pub fn hoare_partition(&mut self, lo: usize, hi: usize) -> usize {
        let pivot_index = self.pick_pivot(lo, hi);
        swap(pivot_index, lo, &mut self.vec);
        let pivot = self.vec[lo];
        let mut i = lo;
        let mut j = hi;
        loop {
            match self.order {
                Order::Ascending => {
                    while i < hi && self.vec[i] < pivot {
                        i += 1;
                    }
                    while j > lo && self.vec[j] > pivot {
                        j -= 1;
                    }
                }
                Order::Descending => {
                    while i < hi && self.vec[i] > pivot {
                        i += 1;
                    }
                    while j > lo && self.vec[j] < pivot {
                        j -= 1;
                    }
                }
            }
            if i >= j {
                return j;
            }
            swap(i, j, &mut self.vec);
            i += 1;
            j -= 1;
        }
    }

    fn sorted(&self) -> Vec<T> {
        let mut v = self.vec.clone();
        match self.order {
            Order::Ascending => v.sort_by(|a, b| a.partial_cmp(b).unwrap()),
            Order::Descending => v.sort_by(|a, b| b.partial_cmp(a).unwrap()),
        }
        v
    }

    pub fn is_sorted(&self) -> bool {
        self.vec == self.sorted()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_cases() -> Vec<(Vec<i32>, Pivot, Order, Strategy)> {
        let orders = [Order::Ascending, Order::Descending];
        let pivots = [Pivot::First, Pivot::Last, Pivot::Middle];
        let strategies = [Strategy::Lomuto, Strategy::Hoare];
        let inputs: Vec<Vec<i32>> = vec![
            vec![3, 1, 4, 1, 5, 9, 2, 6],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![2, 2, 2, 2],
            vec![1],
            vec![7, 3, 9, 1, 8, 5, 2, 6, 0, 4],
        ];
        let mut cases = Vec::new();
        for input in inputs {
            for order in &orders {
                for pivot in &pivots {
                    for strategy in &strategies {
                        cases.push((input.clone(), (*pivot).clone(), (*order).clone(), (*strategy).clone()));
                    }
                }
            }
        }
        cases
    }

    #[test]
    fn sorts_correctly() {
        for (input, pivot, order, strategy) in all_cases() {
            let mut qs = QuickSort::new(input.clone(), pivot.clone(), order.clone(), strategy.clone());
            qs.sort();
            assert!(qs.is_sorted(), "falhou para {:?} | {:?} | {:?} | {:?} -> {:?}",
                input, pivot, order, strategy, qs.vec);
        }
    }
}
