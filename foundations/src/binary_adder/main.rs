fn main() {
    let n: usize = 4;
    let a = [1, 0, 1, 0].to_vec();
    let b = [1, 1, 1, 0].to_vec();
    let c = add(&a, &b, n);
    print!("{:?}", c);
}

fn add(a: &[i32], b: &[i32], n: usize) -> Vec<i32> {
    let mut c = vec![0; n + 1];
    let mut carry: i32 = 0;
    let mut sum: i32;
    for i in (0..n - 1).rev() {
        sum = a[i] + b[i] + carry;
        c[i + 1] = sum % 2;
        c[i] = carry;
        carry = sum / 2;
    }
    return c;
}
