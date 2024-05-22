fn main() {
    let n: usize = 6;
    let mut a = [5, 2, 4, 6, 1, 3];
    BubbleSort::sort(&mut a, n);
    print!("[");
    for elem in a {
        print!(" {} ", elem);
    }
    print!("]");
}

struct BubbleSort {}

impl BubbleSort {
    fn sort(a: &mut [i32], n: usize) {
        for i in 0..n {
            for j in (i + 1..n).rev() {
                if a[j] < a[j - 1] {
                    a.swap(j, j - 1);  // interesting array method
                }
            }
        }
    }
}
