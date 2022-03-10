use image::GenericImageView;
use plotters::prelude::*;

fn main() {
    let mut histogram_r: [i32; 256] = [0; 256];
    let mut histogram_g: [i32; 256] = [0; 256];
    let mut histogram_b: [i32; 256] = [0; 256];

    // Load the image
    let img = image::open("src/index.jpeg").unwrap();

    // Iterate over the coordinates and pixels of the image
    for (_x, _y, pixel) in img.pixels() {
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];
        histogram_r[red as usize] += 1;
        histogram_g[green as usize] += 1;
        histogram_b[blue as usize] += 1;
    }

    area_chart(histogram_r, histogram_g, histogram_b);
}

fn area_chart(red: [i32; 256], green: [i32; 256], blue: [i32; 256]) {
    let root_area =
        BitMapBackend::new("images/image_histogram.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..255, 0..10000)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(red.iter().map(|x| *x)), // The data iter
            0,                                 // Baseline
            &RED.mix(0.2),                     // Make the series opac
        )
        .border_style(&RED), // Make a brighter border
    )
    .unwrap();

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(green.iter().map(|x| *x)), // The data iter
            0,                                   // Baseline
            &GREEN.mix(0.2),                     // Make the series opac
        )
        .border_style(&GREEN), // Make a brighter border
    )
    .unwrap();

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(blue.iter().map(|x| *x)), // The data iter
            0,                                  // Baseline
            &BLUE.mix(0.2),                     // Make the series opac
        )
        .border_style(&BLUE), // Make a brighter border
    )
    .unwrap();
}
