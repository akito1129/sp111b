use crate::image::Image;

use png::{Encoder, ColorType, BitDepth}; //導入需要的結構函數
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn to_ppm(image: Image) -> Result<(), Box<dyn std::error::Error>> { //將函數名稱設為 to_png 並將返回類型改為方便 Debug 的函數
    let file = File::create("out.png")?; //將輸出的文件名稱改為 out.png
    let mut encoder = Encoder::new(file, image.width, image.height); //建立 png 的編碼器並設置長寬跟像素格式
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = BufWriter::new(encoder.write_header()?); //從 png 獲取寫入器 ( encoder ) 並將它包裝為 BufWriter

    for row in 0..image.height { //將每個像素的 RGB 值寫入 writer 中
        for col in 0..image.width {
            let pixel = &image.pixels[row as usize][col as usize];
            writer.write(&[pixel.r, pixel.g, pixel.b])?;
        }
    }
    Ok(())
}