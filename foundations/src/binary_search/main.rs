fn main() {
    let mut a = [6, 5, 7, 3, 2, 1].to_vec();
    a.sort();
    print!("[");
    for &elem in &a {
        print!(" {} ", elem);
    }
    print!("]\n");
    let index = binary_search(&a, 4, 0, a.len());
    println!("{:?}", index);
}

fn binary_search(a: &Vec<i32>, val: i32, p: usize, r: usize) -> Option<usize> {
    if p >= r { return None };
    let q = (p + r) / 2;
    if val == a[q] { return Some(q) }
    if val < a[q] { return binary_search(a, val, p, q); }
    return binary_search(a, val, q + 1, r);
}
