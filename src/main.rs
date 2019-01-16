use rand;
#[macro_use]
extern crate serde_derive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CellValue {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl rand::distributions::Distribution<CellValue> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CellValue {
        let key: u8 = rng.gen_range(0, 6);
        match key {
            0 => CellValue::Zero,
            1 => CellValue::One,
            2 => CellValue::Two,
            3 => CellValue::Three,
            4 => CellValue::Four,
            5 => CellValue::Five,
            _ => panic!("Unexpected random value while generating a CellValue"),
        }
    }
}

const CELAUT_SIZE: usize = 128;
type Universe = [CellValue; CELAUT_SIZE];
// type Rule = Fn(Option<CellValue>, CellValue, Option<CellValue>) -> CellValue;

fn write_new_universe<R>(old_universe: &Universe, new_universe: &mut Universe, rule: R)
where
    R: Fn(Option<CellValue>, CellValue, Option<CellValue>) -> CellValue,
{
    for idx in 0..CELAUT_SIZE {
        let left = if idx > 0 {
            Some(old_universe[idx - 1])
        } else {
            None
        };
        let right = if idx < CELAUT_SIZE - 1 {
            Some(old_universe[idx + 1])
        } else {
            None
        };
        let centre = old_universe[idx];
        new_universe[idx] = rule(left, centre, right);
    }
}

pub struct CelAut {
    universe: Universe,
}

impl CelAut {
    fn random() -> Self {
        let mut universe = [CellValue::Zero; CELAUT_SIZE];
        for idx in 0..CELAUT_SIZE {
            universe[idx] = rand::random();
        }
        CelAut { universe }
    }

    fn advance<R>(&mut self, rule: R)
    where
        R: Fn(Option<CellValue>, CellValue, Option<CellValue>) -> CellValue,
    {
        let old_universe = self.universe.clone();
        write_new_universe(&old_universe, &mut self.universe, rule);
    }
}

mod cmp_table {
    use std::cmp::Ordering;
    use std::fmt;

    use crate::CellValue;

    fn compare_lr(
        left: Option<CellValue>,
        centre: CellValue,
        right: Option<CellValue>,
    ) -> (Ordering, Ordering) {
        let ldiff = match left {
            Some(n) => centre.cmp(&n),
            None => Ordering::Equal,
        };
        let rdiff = match right {
            Some(n) => centre.cmp(&n),
            None => Ordering::Equal,
        };
        (ldiff, rdiff)
    }

    fn ord_to_idx(o: Ordering) -> usize {
        match o {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Table {
        tbl: [[CellValue; 3]; 3],
    }

    impl Table {
        pub fn lookup(
            &self,
            left: Option<CellValue>,
            centre: CellValue,
            right: Option<CellValue>,
        ) -> CellValue {
            let (lc, rc) = compare_lr(left, centre, right);
            let iy = ord_to_idx(lc);
            let ix = ord_to_idx(rc);
            self.tbl[ix][iy]
        }
    }

    impl fmt::Debug for Table {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..3 {
                for x in 0..3 {
                    let v = self.tbl[x][y];
                    write!(formatter, "{:?}", v)?;
                }
                write!(formatter, "\n");
            }
            Ok(())
        }
    }

    impl rand::distributions::Distribution<Table> for rand::distributions::Standard {
        fn sample<R: rand::Rng + ?Sized>(&self, _rng: &mut R) -> Table {
            let mut tbl = [[CellValue::Zero; 3]; 3];
            for x in 0..2 {
                for y in 0..2 {
                    tbl[x][y] = rand::random();
                }
            }
            Table { tbl }
        }
    }

}

mod render {

    fn to_pixel(cell: crate::CellValue) -> image::Rgb<u8> {
        use crate::CellValue;
        let data: [u8; 3] = match cell {
            CellValue::Zero => [0; 3],
            CellValue::One => [51; 3],
            CellValue::Two => [102; 3],
            CellValue::Three => [153; 3],
            CellValue::Four => [204; 3],
            CellValue::Five => [255; 3],
        };
        image::Rgb(data)
    }

    pub fn render_evolution(mut celaut: crate::CelAut, tbl: &crate::cmp_table::Table) {
        use image::ImageBuffer;
        let mut imgbuf = ImageBuffer::new(crate::CELAUT_SIZE as u32, crate::CELAUT_SIZE as u32);
        let rule = |l, c, r| tbl.lookup(l, c, r);
        for y in 0..crate::CELAUT_SIZE {
            for x in 0..crate::CELAUT_SIZE {
                let px = to_pixel(celaut.universe[x as usize]);
                imgbuf.put_pixel(x as u32, y as u32, px);
            }
            celaut.advance(rule);
        }
        imgbuf.save("celaut.png").unwrap();
    }

}

fn get_tbl() -> cmp_table::Table {
    use std::env;
    let argv: Vec<String> = env::args().collect();
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
    let celaut = CelAut::random();
    let tbl: cmp_table::Table = get_tbl();
    render::render_evolution(celaut, &tbl);
}
