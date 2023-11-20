#![feature(portable_simd)]
#![feature(int_roundings)]
use image::{ImageError, Rgb, RgbImage};
use imageproc::map::map_pixels;
use std::{
    ops::{Add, Mul, Sub},
    simd::{f32x2, f32x4, f32x8},
    time::SystemTime,
};

#[derive(Clone, Copy, Debug)]
struct Complex<T: Copy + Clone> {
    real: T,
    imag: T,
}

fn mandel<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy + Clone + Default>(
    c: &Complex<T>,
    iters: usize,
) -> Complex<T> {
    let mut z = Complex {
        real: T::default(),
        imag: T::default(),
    };
    for _ in 0..iters {
        let new_real = z.real * z.real - z.imag * z.imag + c.real;
        let new_imag = z.real * z.imag + z.imag * z.real + c.imag;
        z.real = new_real;
        z.imag = new_imag;
    }
    z
}

fn draw_brot<F: Fn(Vec<(f32, f32)>) -> Vec<Complex<f32>>>(
    dims: (usize, usize),
    filename: String,
    generator: F,
) -> Result<(), ImageError> {
    let (width, height) = dims;

    let start = SystemTime::now();
    let results = generator(
        (0..(width * height))
            .map(|i: usize| {
                let x = i % width;
                let y = i.div_floor(width);
                (
                    ((x as f32) / (width as f32)) * 4.0 - 2.0,
                    ((y as f32) / (height as f32)) * 4.0 - 2.0,
                )
            })
            .collect(),
    );
    let dur = SystemTime::now().duration_since(start).unwrap();
    println!("{:.3}s elapsed", dur.as_secs_f32());

    let img = map_pixels(&RgbImage::new(width as u32, height as u32), |x, y, _| {
        let i = x as usize + y as usize * width;
        let cx = results[i];
        if cx.real.is_nan() || cx.imag.is_nan() || cx.real * cx.real + cx.imag * cx.imag > 4.0 {
            Rgb([255, 255, 255] as [u8; 3])
        } else {
            Rgb([0, 0, 0] as [u8; 3])
        }
    });

    img.save(filename)?;
    println!("saved");

    Ok(())
}

fn main() {
    let dims = (2048, 2048);
    let iters = 200;

    println!("No simd test");
    draw_brot(dims, "no_simd.png".into(), |pixels| {
        pixels
            .into_iter()
            .map(|(r, i)| mandel(&Complex { real: r, imag: i }, iters))
            .collect()
    })
    .unwrap();

    println!("2x simd test");
    draw_brot(dims, "2x_simd.png".into(), |pixels| {
        pixels
            .chunks(2)
            .map(|chunk| {
                let (reals, imags): (Vec<f32>, Vec<f32>) =
                    chunk.iter().map(ToOwned::to_owned).unzip();
                let result = mandel(
                    &Complex {
                        real: f32x2::from_array(reals.try_into().unwrap()),
                        imag: f32x2::from_array(imags.try_into().unwrap()),
                    },
                    iters,
                );
                result
                    .real
                    .as_array()
                    .iter()
                    .zip(result.imag.as_array().iter())
                    .map(|(r, i)| Complex { real: *r, imag: *i }.to_owned())
                    .collect::<Vec<Complex<f32>>>()
            })
            .flatten()
            .collect()
    })
    .unwrap();

    println!("4x simd test");
    draw_brot(dims, "4x_simd.png".into(), |pixels| {
        pixels
            .chunks(4)
            .map(|chunk| {
                let (reals, imags): (Vec<f32>, Vec<f32>) =
                    chunk.iter().map(ToOwned::to_owned).unzip();
                let result = mandel(
                    &Complex {
                        real: f32x4::from_array(reals.try_into().unwrap()),
                        imag: f32x4::from_array(imags.try_into().unwrap()),
                    },
                    iters,
                );
                result
                    .real
                    .as_array()
                    .iter()
                    .zip(result.imag.as_array().iter())
                    .map(|(r, i)| Complex { real: *r, imag: *i }.to_owned())
                    .collect::<Vec<Complex<f32>>>()
            })
            .flatten()
            .collect()
    })
    .unwrap();

    println!("8x simd test");
    draw_brot(dims, "8x_simd.png".into(), |pixels| {
        pixels
            .chunks(8)
            .map(|chunk| {
                let (reals, imags): (Vec<f32>, Vec<f32>) =
                    chunk.iter().map(ToOwned::to_owned).unzip();
                let result = mandel(
                    &Complex {
                        real: f32x8::from_array(reals.try_into().unwrap()),
                        imag: f32x8::from_array(imags.try_into().unwrap()),
                    },
                    iters,
                );
                result
                    .real
                    .as_array()
                    .iter()
                    .zip(result.imag.as_array().iter())
                    .map(|(r, i)| Complex { real: *r, imag: *i }.to_owned())
                    .collect::<Vec<Complex<f32>>>()
            })
            .flatten()
            .collect()
    })
    .unwrap();
}
