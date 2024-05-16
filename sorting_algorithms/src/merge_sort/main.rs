fn main() {
    let n: usize = 8;
    let mut a = [12, 3, 7, 9, 14, 6, 11, 2];
    MergeSort::sort(&mut a, 0, n - 1);
    print!("[");
    for elem in a {
        print!(" {} ", elem);
    }
    print!("]");
}

struct MergeSort {}

impl MergeSort {
    fn sort(a: &mut [i32], p: usize, r: usize) {
        if p >= r { return; }
        let q = (p + r) / 2;
        Self::sort(a, p, q);
        Self::sort(a, q + 1, r);
        Self::merge(a, p, q, r);
    }

    fn merge(a: &mut [i32], p: usize, q: usize, r: usize) {
        let n_l = q - p + 1;
        let n_r = r - q;
        let mut l = vec![0; n_l];
        let mut r = vec![0; n_r];
        for i in 0..n_l {
            l[i] = a[p + i];
        }
        for j in 0..n_r {
            r[j] = a[q + j + 1];
        }
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k = p;
        while i < n_l && j < n_r {
            if l[i] <= r[j] {
                a[k] = l[i];
                i += 1;
            } else {
                a[k] = r[j];
                j += 1;
            }
            k += 1;
        }
        while i < n_l {
            a[k] = l[i];
            i += 1;
            k += 1;
        }
        while j < n_r {
            a[k] = r[j];
            j += 1;
            k += 1;
        }
    }
}
