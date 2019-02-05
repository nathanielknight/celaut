use celaut::{convert, diff_table, render, *};
use rand::{self, Rng};

use image::ImageBuffer;

fn random_cell_value() -> CellValue {
    let key: u8 = rand::thread_rng().gen_range(0, CELL_LIMIT as u8);
    match key {
        0 => CellValue::Zero,
        1 => CellValue::One,
        2 => CellValue::Two,
        3 => CellValue::Three,
        _ => panic!("random::gen_range produced an out-of-bounds value"),
    }
}

fn random_celaut() -> CelAut {
    let mut universe = [CellValue::Zero; CELAUT_SIZE];
    for idx in 0..CELAUT_SIZE {
        universe[idx] = random_cell_value();
    }
    CelAut::new(universe)
}

fn random_diff_table() -> diff_table::Table {
    let mut tbl = [[CellValue::Zero; TABLE_SIZE]; TABLE_SIZE];
    for x in 0..TABLE_SIZE {
        for y in 0..TABLE_SIZE {
            tbl[x][y] = random_cell_value();
        }
    }
    diff_table::Table::new(tbl)
}

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
    let mut imgbuf: BoringBuffer = BoringBuffer {
        buffer: ImageBuffer::new(CELAUT_SIZE as u32, CELAUT_SIZE as u32),
    };
    render::render_evolution(celaut, tbl, &mut imgbuf);
    imgbuf.buffer.save("celaut.png").unwrap();
}

fn get_tbl() -> diff_table::Table {
    use std::env;
    let argv: Vec<String> = env::args().collect();
    if argv.len() > 1 {
        let src = &argv[1];
        convert::table_from_str(src).unwrap()
    } else {
        let tbl = random_diff_table();
        println!("{}", convert::table_to_string(&tbl));
        tbl
    }
}

fn main() {
    let mut celaut = random_celaut();
    let tbl: diff_table::Table = get_tbl();
    render_image(&mut celaut, &tbl);
}
