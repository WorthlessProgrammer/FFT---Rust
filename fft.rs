use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

#[derive(Debug)]
struct ImgVec {
    pixels: [u32; 256*256],
    stride: usize
}

impl ImgVec {

    pub fn new() -> ImgVec {
        ImgVec{pixels: [0; 256*256], stride: 256}
    }

    pub fn draw_square(&mut self, r: usize, cx: usize, cy: usize, color: u32) {
        let x0 = cx-r;
        let y0 = cy-r;
        
        for i in y0..cy+r {
            for j in x0..cx+r {
                let index = j*self.stride+i;
                if index <= 256*256-1 {
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
        for i in 0..self.stride*self.stride {
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
}

fn main() {
    let mut im = ImgVec::new();
    im.draw_square(8, 128, 128, 0x181818FF);
    im.dump_bmp("test.ppm".to_string()).unwrap();
}
