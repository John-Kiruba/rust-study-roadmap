use std::cmp::Ordering;

pub fn exponential_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let mut upper = 1;

    while upper < len && arr[upper].cmp(item) != Ordering::Greater {
        upper *= 2;
    }

    upper = upper.min(len);

    let mut lower = upper / 2;

    while lower < upper {
        let mid = lower + (upper - lower) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => upper = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => lower = mid + 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $tc:expr),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $tc;

                    if let Some(expected_index) = expected {
                        assert_eq!(arr[expected_index], item);
                    }

                    assert_eq!(exponential_search(&item, arr), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_strings_asc_start: ("a", &["a", "b", "c"], Some(0)),
    }
}
