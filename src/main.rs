fn main() {

    let mut vecval = vec![];

    let n = 3;
    for i in 0..n {
        vecval.push((i,i,2.));
    }

    vecval[0] = (0,0,100.);

    let mut m = sparse21::Matrix::from_entries(vecval);

    // m.add_element(0, 0, 100.);

    // println!("{:?}",m.get(0,0));
    displaymat(n, &m);

    let soln = m.solve(vec![1.,1.,1.]).unwrap();
    println!("{:?}", soln);
    // => vec![5.0, 3.0, -2.0]

    // plot2d example
    let lx = 1.;
    let ly = 2.;

    let nx = 2;
    let ny = 4;

    let dx = lx / nx as f64;
    let dy = ly / ny as f64;

    fn f(x: f64, y: f64) -> f64 {
        //let pi = std::f64::consts::PI;
        x + y
    }

    let xp: Vec<f64> = (0..nx + 1).map(|i| i as f64 * dx).collect();
    let yp: Vec<f64> = (0..ny + 1).map(|i| i as f64 * dy).collect();

    let mut zp: Vec<f64> = vec![];

    yp.iter().for_each(|y| {
        xp.iter().for_each(|x| {
            zp.push(f(*x, *y));
        });
    });

    plotpy(xp, yp, zp);
}

fn displaymat(n:usize, m: & sparse21::Matrix){
    for i in 0..n {
        for j in 0..n {
            let val = match m.get(i,j) {
                Some(v) => v,
                None => 0.,
            };
            print!("{} ",val);
        }
        println!();
    }
}

/// Plot a 2D data set using matplotlib
fn plotpy(xp: Vec<f64>, yp: Vec<f64>, zp: Vec<f64>) {
    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;
    {
        let meshfile = File::create("plotpy.dat").unwrap();
        let mut meshfile = BufWriter::new(meshfile); // create a buffer for faster writes...
        xp.iter().for_each(|x| {
            writeln!(meshfile, "{}", x).unwrap();
        });
        writeln!(meshfile, "").unwrap();
        yp.iter().for_each(|y| {
            writeln!(meshfile, "{}", y).unwrap();
        });
        writeln!(meshfile, "").unwrap();
        zp.iter().for_each(|z| {
            writeln!(meshfile, "{}", z).unwrap();
        });
    } // ensures that the file is closed

    use std::process::Command;
    Command::new("python3")
        .arg("src/plot.py")
        .status()
        .expect("plot failed !");
}
