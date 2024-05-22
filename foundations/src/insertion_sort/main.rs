fn main() {
    let n = 6;
    let mut a = [5, 2, 4, 6, 1, 3];
    InsertionSort::sort_with_binary_search(&mut a, n);
    print!("[");
    for elem in a {
        print!(" {} ", elem);
    }
    print!("]");
}

struct InsertionSort {}

impl InsertionSort {
    fn sort(a: &mut [i32], n: usize) {
        for i in 1..n {
            let key = a[i];
            let mut j = i;
            while j > 0 && a[j - 1] > key {
                a[j] = a[j - 1];
                j = j - 1;  // an usize can't be negative
            }
            a[j] = key;
        }
    }

    fn recursive_sort(a: &mut [i32], n: usize) {
        if n == 1 { return; }
        Self::recursive_sort(a, n - 1);
        let key: i32 = a[n - 1];
        for i in (0..n - 1).rev() {
            if key < a[i] {
                a[i + 1] = a[i];
                a[i] = key;
            }
        }
    }

    fn sort_with_binary_search(a: &mut [i32], n: usize) {
        for i in 1..n {
            let key = a[i];
            let index = Self::binary_insert(a, key, 0, i);
            let mut j = i;
            while j > index {
                a[j] = a[j - 1];
                j = j - 1;  // an usize can't be negative
            }
            a[j] = key;
        }
    }

    fn binary_insert(a: &[i32], key: i32, p: usize, r: usize) -> usize {
        let q = (p + r) / 2;
        if p >= r { return p; }
        if key == a[q] {return q;}
        if key < a[q] {return Self::binary_insert(a, key, p, q)}
        return Self::binary_insert(a, key, q + 1, r);
    }
}
