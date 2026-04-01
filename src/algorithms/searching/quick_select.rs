pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
    if left == right {
        return list[left];
    }

    let p = partition(list, left, right);

    if index == p {
        list[p]
    } else if index < p {
        quick_select(list, left, p.saturating_sub(1), index)
    } else {
        quick_select(list, p + 1, right, index)
    }
}

fn partition(list: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = list[right];
    let mut p = left;

    for i in left..right {
        if list[i] < pivot {
            list.swap(p, i);
            p += 1;
        }
    }
    list.swap(p, right);
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr1 = [2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr1, 0, 3, 1), 3);
        let mut arr2 = [2, 5, 9, 12, 16];
        assert_eq!(quick_select(&mut arr2, 1, 3, 2), 9);
        let mut arr2 = [0, 3, 8];
        assert_eq!(quick_select(&mut arr2, 0, 0, 0), 0);
    }
}
