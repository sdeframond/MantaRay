pub fn cmp_float<F: PartialOrd>(f1: F, f2: F) -> Ordering {
    match f1.partial_cmp(&f2) {
        None => Less,
        Some(ord) => ord
    }
}