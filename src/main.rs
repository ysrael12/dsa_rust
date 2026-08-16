mod sort; 
mod utils;
use sort::quick_sort::{QuickSort, Pivot, Order, Strategy};

fn main() {

    let mut quick_sort = QuickSort::new(vec![3, 1, 4, 1, 5, 9, 2, 6], Pivot::Last, Order::Ascending, Strategy::Lomuto);
    quick_sort.sort();
    println!("{:?}", quick_sort.vec);
}
