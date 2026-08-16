pub fn is_ascending<T: PartialOrd>(i : usize, j : usize, vec : &Vec<T>) -> bool {
    return vec[i] <= vec[j];
}

pub fn is_descending<T: PartialOrd>(i : usize, j : usize, vec : &Vec<T>) -> bool {
    return vec[i] >= vec[j];
}

pub fn swap<T: Copy>(i : usize, j : usize, vec : &mut Vec<T>) {
    let temp: T = vec[i];
    vec[i] = vec[j];
    vec[j] = temp;
}