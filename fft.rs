#[derive(Debug)]
struct ImgMat {
    pixels: [u32; 256*256],
    size: usize
}

impl ImgMat {

    pub fn new() -> ImgMat {
        ImgMat{pixels: [0; 256*256], size: 256}
    }

    pub fn draw_cricle(&mut self, r: usize, cx: usize, cy: usize) {
        assert!(cx <= self.size-1, "ERROR: cx too big for this img")
        assert!(cy <= self.size-1, "ERROR: cy too big for this img")
    }
}

fn main() {
    let mut im = ImgMat::new();
    im.draw_cricle(8, 128, 128);
}
