use image::{GenericImageView, ImageBuffer, RgbImage};
use plotters::prelude::*;

fn main() {
    let mut histogram_r: [usize; 256] = [0; 256];
    let mut histogram_g: [usize; 256] = [0; 256];
    let mut histogram_b: [usize; 256] = [0; 256];

    // Load the image
    let img = image::open("src/index.jpeg").unwrap();
    // get the width and the height
    let (x, y) = img.dimensions();

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.pixels() {
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];
        histogram_r[red as usize] += 1;
        histogram_g[green as usize] += 1;
        histogram_b[blue as usize] += 1;
    }

    println!("{:?}", histogram_r);

    img.save("test.png");

    area_chart();

    // The dimensions method returns the images width and height.
    println!("x: {} , y: {}", x, y);
}

fn area_chart() {
    let root_area = BitMapBackend::new("images/2.7.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter().map(|x| *x)), // The data iter
            0,                                  // Baseline
            &RED.mix(0.2),                      // Make the series opac
        )
        .border_style(&RED), // Make a brighter border
    )
    .unwrap();
}
