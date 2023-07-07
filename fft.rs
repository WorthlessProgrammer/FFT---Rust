use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

mod complex;
use complex::ComplexNum;

#[derive(Debug)]
struct ImgVec<T> {
    pixels: [T; IMG_SZ],
    stride: usize
}

impl ImgVec<ComplexNum> {
    pub fn new() -> ImgVec<ComplexNum> {
        ImgVec{pixels: [ComplexNum{real: 0.0, img: 0.0}; IMG_SZ], stride: IMG_STRIDE}
    }
}

impl ImgVec<f32> {
    pub fn new() -> ImgVec<f32> {
        ImgVec{pixels: [0.0; IMG_SZ], stride: IMG_STRIDE}
    }
}

impl ImgVec<u32> {

    pub fn new() -> ImgVec<u32> {
        ImgVec{pixels: [0; IMG_SZ], stride: IMG_STRIDE}
    }

    pub fn draw_square(&mut self, r: usize, cx: usize, cy: usize, color: u32) {
        let x0 = cx-r;
        let y0 = cy-r;
        
        for i in y0..cy+r {
            for j in x0..cx+r {
                let index = j*self.stride+i;
                if index <= IMG_SZ-1 {
                    self.pixels[index] = color;
                }
            }
        }
    }

    pub fn dump_bmp(&self, file_name: String) -> std::io::Result<()> {
        let file = File::create(file_name)?;
        let mut bwiter = BufWriter::new(file);

        // Write the header
        bwiter.write(b"P3 \n")?;
        bwiter.write_all(self.stride.to_string().as_bytes())?;
        bwiter.write(b" ")?;
        bwiter.write_all(self.stride.to_string().as_bytes())?;
        bwiter.write(b" \n")?;
        bwiter.write_all(255.to_string().as_bytes())?;
        bwiter.write(b" \n")?;
        
        // Write image
        for i in 0..IMG_SZ {
            let r = (self.pixels[i] >> 8*0) & 0x000000FF as u32;
            let g = (self.pixels[i] >> 8*1) & 0x000000FF as u32;
            let b = (self.pixels[i] >> 8*2) & 0x000000FF as u32;
            bwiter.write_all(r.to_string().as_bytes())?;
            bwiter.write(b" ")?;
            bwiter.write_all(g.to_string().as_bytes())?;
            bwiter.write(b" ")?;
            bwiter.write_all(b.to_string().as_bytes())?;
            bwiter.write(b" \n")?;
        }

        Ok(())
    }
    
    pub fn rgb2gray(&self) -> ImgVec<f32> {
        let mut gray_img = ImgVec::<f32>::new();
        for i in 0..IMG_SZ {
            let r = ((self.pixels[i] >> 8*0) & 0x000000FF as u32) as f32;
            let g = ((self.pixels[i] >> 8*1) & 0x000000FF as u32) as f32;
            let b = ((self.pixels[i] >> 8*2) & 0x000000FF as u32) as f32;
            gray_img.pixels[i] = 0.299*r + 0.587*g + 0.114*b;
        }
        gray_img
    }
}

const IMG_SZ: usize = 256*256;
const IMG_STRIDE: usize = 256;

fn main() {
    let mut im = ImgVec::<u32>::new();
    im.draw_square(8, 128, 128, 0x181818FF);
    im.dump_bmp("test.ppm".to_string()).unwrap();
    let gray_p = im.rgb2gray();
}

/*
fn main() {
    let a = ComplexNum::new(3.0, 2.0);
    println!("{:?}", complex::exp(5.0, a));
}
*/
