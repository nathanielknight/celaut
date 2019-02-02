use celaut::*;
use crate::diff_table;
use crate::render;

use image::ImageBuffer;

struct BoringBuffer {
    buffer: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
}

impl render::Target for BoringBuffer {
    fn set_value(&mut self, x: u32, y: u32, val: CellValue) {
        let px = to_pixel(val);
        self.buffer.put_pixel(x, y, px);
    }
}

fn to_pixel(cell: CellValue) -> image::Rgb<u8> {
    const RATIO: f32 = 255.0 / (CELL_LIMIT - 1) as f32;
    let intensity: f32 = cell.to_f32() * RATIO;
    let data: [u8; 3] = [intensity as u8; 3];
    image::Rgb(data)
}

pub fn render_image(celaut: &mut CelAut, tbl: &crate::diff_table::Table) {
    let mut imgbuf: BoringBuffer =
        BoringBuffer{ buffer: ImageBuffer::new(CELAUT_SIZE as u32, CELAUT_SIZE as u32) };
    render::render_evolution(celaut, tbl, &mut imgbuf);
    imgbuf.buffer.save("celaut.png").unwrap();
}


fn get_tbl() -> diff_table::Table {
    use std::env;
    let argv: Vec<String> = env::args().collect();
    dbg!(&argv);
    if argv.len() > 1 {
        let src = &argv[1];
        serde_json::from_str(src).unwrap()
    } else {
        let tbl = rand::random();
        println!("{}", serde_json::to_string(&tbl).unwrap());
        tbl
    }
}

fn main() {
    let mut celaut = CelAut::random();
    let tbl: diff_table::Table = get_tbl();
    render_image(&mut celaut, &tbl);
}
