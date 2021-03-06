fn main() {
    // plot2d example
    let lx = 1.;
    let ly = 1.;

    let nx = 50;
    let ny = 50;

    let dx = lx / nx as f64;
    let dy = ly / ny as f64;

    let mut vecval = vec![];

    let n = (nx + 1) * (ny + 1);
    for k in 0..n {
        let i = k % (nx + 1);
        let j = k / (nx + 1);
        if i == 0 || i == nx || j == 0 || j == ny {
            vecval.push((k, k, 1e20));
        } else {
            vecval.push((k, k, 4. / dx / dy));
        }
    }

    for i in 0..nx {
        for j in 0..ny + 1 {
            let k1 = j * (nx + 1) + i;
            let k2 = j * (nx + 1) + i + 1;
            vecval.push((k1, k2, -1. / dx / dx));
            vecval.push((k2, k1, -1. / dx / dx));
        }
    }

    for j in 0..ny {
        for i in 0..nx + 1 {
            let k1 = j * (nx + 1) + i;
            let k2 = (j+1) * (nx + 1) + i;
            vecval.push((k1, k2, -1. / dx / dx));
            vecval.push((k2, k1, -1. / dx / dx));
        }
    }

    let mut m = skyrs::Sky::new(vecval);

    //displaymat(n, &m);

    let zp = m.solve(vec![1.; n]).unwrap();
    //println!("{:?}", soln);
    // => vec![5.0, 3.0, -2.0]

    // plot2d example

    let xp: Vec<f64> = (0..nx + 1).map(|i| i as f64 * dx).collect();
    let yp: Vec<f64> = (0..ny + 1).map(|i| i as f64 * dy).collect();

    plotpy(xp, yp, zp);
}

#[allow(dead_code)]
fn displaymat(m: &skyrs::Sky) {
    println!("Matrix=");
    m.print_coo();
    println!("L-I+U");
    m.print_lu();
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
