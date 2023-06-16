#![allow(non_snake_case)]
mod png;
use crate::image::Image;

use std::io::BufReader;
use std::fs::File;

use std::f32::consts::PI;

use crate::primitives::*;
use crate::reader::data_reader;
use crate::image::Color;

fn png_function();

fn cc(i: usize, j: usize) -> f32 {
    if i == 0 && j == 0 {
        return 1.0 / 2.0;
    } else if i == 0 || j == 0 {
        return 1.0 / (2.0 as f32).sqrt();
    } else {
        return 1.0;
    }
}

fn chomp(x: f32) -> u8 {
    if x >= 255.0 {
        return 255;
    } else if x <= 0.0 {
        return 0;
    } else {
        return x.round() as u8;
    }
}

const ZZ: [[usize; 8]; 8] = [
    [ 0,  1,  5,  6, 14, 15, 27, 28 ],
    [ 2,  4,  7, 13, 16, 26, 29, 42 ],
    [ 3,  8, 12, 17, 25, 30, 41, 43 ],
    [ 9, 11, 18, 24, 31, 40, 44, 53 ],
    [ 10, 19, 23, 32, 39, 45, 52, 54 ],
    [ 20, 22, 33, 38, 46, 51, 55, 60 ],
    [ 21, 34, 37, 47, 50, 56, 59, 61 ],
    [ 35, 36, 48, 49, 57, 58, 62, 63 ]
];

struct MCUWrap<'a> {
    mcu: MCU,
    jpeg_meta_data: &'a JPEGMetaData,
} 

impl<'a> MCUWrap<'a> {
    fn new(mcu: MCU, jpeg_meta_data: &'a JPEGMetaData) -> MCUWrap<'a> {
        return MCUWrap{ mcu, jpeg_meta_data };
    }
    fn display(&mut self) {
        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        let m = ["Y", "Cb", "Cr"];
        for id in 0..3 {
            let c_info = &component_infos[id];
            for h in 0..(c_info.vertical_sampling as usize) {
                for w in 0..(c_info.horizontal_sampling as usize) {
                    println!("------ {} 顏色分量 {} {} ------", m[id], h, w);
                    let block = &self.mcu[id][h][w];
                    for i in 0..8 {
                        for j in 0..8 {
                            print!("{} ", block[i][j]);
                        }
                        println!("");
                    }
                }
            }
        }
    }
    fn dequantize(&mut self) {
        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        let quant_tables = &self.jpeg_meta_data.quant_tables;
        for id in 0..3 {
            let c_info = &component_infos[id];
            for h in 0..(c_info.vertical_sampling as usize) {
                for w in 0..(c_info.horizontal_sampling as usize) {

                    for i in 0..8 {
                        for j in 0..8 {
                            self.mcu[id][h][w][i][j] *= quant_tables[c_info.quant_table_id as usize][i*8 + j];
                        }
                    }

                }
            }
        }
    }
    fn zigzag(&mut self) {
        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        for id in 0..3 {
            let c_info = &component_infos[id];
            for h in 0..(c_info.vertical_sampling as usize) {
                for w in 0..(c_info.horizontal_sampling as usize) {

                    let mut tmp: [[f32; 8]; 8] = Default::default();
                    for i in 0..8 {
                        for j in 0..8 {
                            tmp[i][j] = self.mcu[id][h][w][ZZ[i][j] / 8][ZZ[i][j] % 8];
                        }
                    }
                    self.mcu[id][h][w] = tmp;

                }
            }
        }
    }
    // NOTE: idct 直接照定義展開
    // 可嘗試其他優化方法
    fn idct(&mut self) {
        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        for id in 0..3 {
            let c_info = &component_infos[id];
            for h in 0..(c_info.vertical_sampling as usize) {
                for w in 0..(c_info.horizontal_sampling as usize) {

                    let mut tmp: [[f32; 8]; 8] = Default::default();
                    for i in 0..8 {
                        for j in 0..8 {
                            for x in 0..8 {
                                for y in 0..8 {
                                    let i_cos = ((2*i+1) as f32 * PI / 16.0 * x as f32).cos();
                                    let j_cos =((2*j+1) as f32 * PI / 16.0 * y as f32).cos();
                                    tmp[i][j] += cc(x, y) * self.mcu[id][h][w][x][y] * i_cos * j_cos;
                                }
                            }
                            tmp[i][j] /= 4.0;
                        }
                    }
                    self.mcu[id][h][w] = tmp;

                }
            }
        }
    }
    // NOTE: dequantize, zigzag, idct 的外層迴圈其實是一樣的
    // 把它們寫在一起可以更有效率、也可以節省很多程式碼行數
    // 但此處爲了能夠將每個階段的狀態都打印出來，將每個階段都寫成函式
    fn decode(&mut self) {
        self.dequantize();
        self.zigzag();
        self.idct();
    }
    fn show_all_stage(&mut self) {
        println!("---------------- 未經處理 ----------------");
        self.display();
        self.dequantize();
        println!("---------------- 反量化之後 ----------------");
        self.display();
        self.zigzag();
        println!("---------------- zigzag 之後 ----------------");
        self.display();
        self.idct();
        println!("---------------- 反向餘弦變換之後 ----------------");
        self.display();
    }

    fn toRGB(&mut self) -> Vec<Vec<Color>> {
        self.decode();

        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        let max_vertical_sampling = sof_info.max_vertical_sampling;
        let max_horizontal_sampling = sof_info.max_horizontal_sampling;
        let mcu_height = 8 * max_vertical_sampling;
        let mcu_width = 8 * max_horizontal_sampling;

        let mut ret = vec![vec![Color::RGB(0, 0, 0); mcu_width as usize]; mcu_height as usize];
        for i in 0..mcu_height {
            for j in 0..mcu_width {
                let mut YCbCr = [0.0; 3];
                for id in 0..3 {
                    let vh = (i * component_infos[id].vertical_sampling / max_vertical_sampling) as usize;
                    let uh = (j * component_infos[id].horizontal_sampling / max_horizontal_sampling) as usize;

                    for m in 0..component_infos[id].vertical_sampling {
                        for n in 0..component_infos[id].horizontal_sampling {
                            let cur = self.mcu[id][vh + m as usize][uh + n as usize];
                            let pos = ZZ[m * 8 + n];
                            for k in 0..64 {
                                let row = pos[k] / 8;
                                let col = pos[k] % 8;
                                let cos_a = (2.0 * PI * row as f32 / 16.0).cos();
                                let cos_b = (2.0 * PI * col as f32 / 16.0).cos();
                                YCbCr[id] += cur[row][col] * cc(row, col) * cos_a * cos_b;
                            }
                        }
                    }
                }

                let R = chomp(YCbCr[0] + 1.402 * (YCbCr[2] - 128.0));
                let G = chomp(YCbCr[0] - 0.344136 * (YCbCr[1] - 128.0) - 0.714136 * (YCbCr[2] - 128.0));
                let B = chomp(YCbCr[0] + 1.772 * (YCbCr[1] - 128.0));

                ret[i as usize][j as usize] = Color::RGB(R, G, B);
            }
        }

        return ret
    }
}

fn save_as_png(image: Image, filename: &str) -> Result<(), std::io::Error> {
    let width = image.width;
    let height = image.height;
    let mut buffer = Vec::new();

    for row in image.pixels {
        for pixel in row {
            match pixel {
                Color::RGB(r, g, b) => {
                    buffer.push(r);
                    buffer.push(g);
                    buffer.push(b);
                }
            }
        }
    }

    let file = File::create(filename)?;
    let encoder = png::Encoder::new(file, width, height);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&buffer)?;

    Ok(())
}

fn main() {
    let file = File::open("./data/input.jpg").expect("Failed to open input.jpg");
    let reader = BufReader::new(file);
    let (meta_data, mcu_data) = data_reader(reader);

    let mut mcu_wrap = MCUWrap {
        mcu: mcu_data,
        jpeg_meta_data: &meta_data,
    };

    let image = Image::new(meta_data.sof_info.image_width, meta_data.sof_info.image_height);
    let rgb_pixels = mcu_wrap.toRGB();
    let image_with_pixels = Image {
        pixels: rgb_pixels,
        width: meta_data.sof_info.image_width,
        height: meta_data.sof_info.image_height,
    };

    save_as_png(image_with_pixels, "output.png").expect("Failed to save PNG file");
}