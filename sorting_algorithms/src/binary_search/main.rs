fn main() {
    let mut a = [6, 5, 7, 3, 2, 1].to_vec();
    a.sort();
    print!("[");
    for &elem in &a {
        print!(" {} ", elem);
    }
    print!("]\n");
    let index = binary_search(&a, 7, 0, a.len() - 1);
    println!("{:?}", index);
}

fn binary_search(a: &Vec<i32>, val: i32, p: usize, r: usize) -> Option<usize> {
    if p > r { return None };
    let q = (p + r) / 2;
    return if val == a[q] { Some(q) }
    else if val < a[q] { binary_search(a, val, p, q - 1) }
    else { binary_search(a, val, q + 1, r) }
}
