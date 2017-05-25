include!("header.rs");

#[test]
fn n_columns() {
    let a = random_owned(3, 2, true);
    let (n, v) = normalize(a.clone(), NormalizeAxis::Column);
    all_close_l2(&n.dot(&from_diag(&v)), &a, 1e-7).unwrap();
}

#[test]
fn n_rows() {
    let a = random_owned(3, 2, true);
    let (n, v) = normalize(a.clone(), NormalizeAxis::Row);
    all_close_l2(&from_diag(&v).dot(&n), &a, 1e-7).unwrap();
}