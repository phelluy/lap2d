fn main() {
    let mut m = sparse21::Matrix::from_entries(vec![
        (0, 0, 1.0),
        (0, 1, 1.0),
        (0, 2, 1.0),
        (1, 1, 2.0),
        (1, 2, 5.0),
        (2, 0, 2.0),
        (2, 1, 5.0),
        (2, 2, -1.0),
    ]);

    let soln = m.solve(vec![6.0, -4.0, 27.0]).unwrap(); 
    println!("{:?}", soln);
    // => vec![5.0, 3.0, -2.0]
}
