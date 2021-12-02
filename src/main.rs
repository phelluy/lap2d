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

    // plot2d example
    let Lx = 1.;
    let Ly = 2.;

    let nx = 30;
    let ny = 120;

    let dx = Lx / nx as f64;
    let dy = Ly / ny as f64;

    fn f(x: f64, y: f64) -> f64 {
        let pi = std::f64::consts::PI;
        x * x * x + (2. * pi * y).sin()
    }

    let xp: Vec<f64> = (0..nx + 1).map(|i| i as f64 * dx).collect();
    let yp: Vec<f64> = (0..ny + 1).map(|i| i as f64 * dy).collect();

    use std::fs::File;
    use std::io::BufWriter;
    use std::io::{Error, Write};
    let meshfile = File::create("trans.dat").unwrap();
    let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...
    xp.iter().for_each(|x| {
        writeln!(meshfile, "{}", x).unwrap();
    });
    writeln!(meshfile, "").unwrap();
    yp.iter().for_each(|y| {
        writeln!(meshfile, "{}", y).unwrap();
    });
    writeln!(meshfile, "").unwrap();
    yp.iter().for_each(|y| {
        xp.iter().for_each(|x| {
            writeln!(meshfile, "{}", f(*x,*y)).unwrap();
        });
    });

}
