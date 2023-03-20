use std::string;

#[derive(Debug)]
struct ImgVec {
    pixels: [u32; 256*256],
    stride: usize
}

impl ImgVec {

    pub fn new() -> ImgVec {
        ImgVec{pixels: [0; 256*256], stride: 256}
    }

    pub fn draw_cricle(&mut self, r: usize, cx: usize, cy: usize, color: u32) {
        let x0 = cx-r;
        let y0 = cy-r;
        
        for i in y0..cy+r {
            for j in x0..cx+r {
                let index = j*self.stride+i;
                if 0 <= index && index <= 256*256-1 {
                    self.pixels[index] = color;
                }
            }
        }
    }

    pub fn dump_bmp(&self, file_name: String) {
    
    }
}

fn main() {
    let mut im = ImgVec::new();
    im.draw_cricle(8, 128, 128, 0x181818FF);
}
