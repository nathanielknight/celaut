use rand;
#[macro_use]
extern crate serde_derive;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CellValue {
    Zero,
    One,
    Two,
    Three,
}

pub const CELL_LIMIT: usize = 4;
const TABLE_SIZE: usize = 2 * CELL_LIMIT - 1;

impl rand::distributions::Distribution<CellValue> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CellValue {
        let key: u8 = rng.gen_range(0, CELL_LIMIT as u8);
        match key {
            0 => CellValue::Zero,
            1 => CellValue::One,
            2 => CellValue::Two,
            3 => CellValue::Three,
            _ => panic!("Random::gen_range produced an out-of-bounds value"),
        }
    }
}

impl CellValue {
    fn to_i8(&self) -> i8 {
        match self {
            CellValue::Zero => 0,
            CellValue::One => 1,
            CellValue::Two => 2,
            CellValue::Three => 3,
        }
    }

    pub fn difference(&self, &other: &CellValue) -> i8 {
        self.to_i8() - other.to_i8()
    }

    pub fn to_f32(&self) -> f32 {
        self.to_i8() as f32
    }
}

pub const CELAUT_SIZE: usize = 128;
pub type Universe = [CellValue; CELAUT_SIZE];
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
    pub fn random() -> Self {
        let mut universe = [CellValue::Zero; CELAUT_SIZE];
        for idx in 0..CELAUT_SIZE {
            universe[idx] = rand::random();
        }
        CelAut { universe }
    }

    pub fn advance<R>(&mut self, rule: R)
    where
        R: Fn(Option<CellValue>, CellValue, Option<CellValue>) -> CellValue,
    {
        let old_universe = self.universe.clone();
        write_new_universe(&old_universe, &mut self.universe, rule);
    }
}

pub mod diff_table {
    use crate::CellValue;
    use crate::{CELL_LIMIT, TABLE_SIZE};
    use std::fmt;

    fn compare_lr(
        left: Option<CellValue>,
        centre: CellValue,
        right: Option<CellValue>,
    ) -> (i8, i8) {
        let ldiff = match left {
            Some(n) => centre.difference(&n),
            None => 0,
        };
        let rdiff = match right {
            Some(n) => centre.difference(&n),
            None => 0,
        };
        (ldiff, rdiff)
    }

    #[derive(Serialize, Deserialize)]
    pub struct Table {
        tbl: [[CellValue; TABLE_SIZE]; TABLE_SIZE],
    }

    impl Table {
        pub fn lookup(
            &self,
            left: Option<CellValue>,
            centre: CellValue,
            right: Option<CellValue>,
        ) -> CellValue {
            let (cl, cr) = compare_lr(left, centre, right);
            let il = (cl + CELL_LIMIT as i8 - 1) as usize;
            let ir = (cr + CELL_LIMIT as i8 - 1) as usize;
            self.tbl[il][ir]
        }
    }

    impl fmt::Debug for Table {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            for y in 0..TABLE_SIZE {
                for x in 0..TABLE_SIZE {
                    let v = self.tbl[x][y];
                    write!(formatter, "{:?}", v)?;
                }
                write!(formatter, "\n").unwrap();
            }
            Ok(())
        }
    }

    impl rand::distributions::Distribution<Table> for rand::distributions::Standard {
        fn sample<R: rand::Rng + ?Sized>(&self, _rng: &mut R) -> Table {
            let mut tbl = [[CellValue::Zero; TABLE_SIZE]; TABLE_SIZE];
            for x in 0..TABLE_SIZE {
                for y in 0..TABLE_SIZE {
                    tbl[x][y] = rand::random();
                }
            }
            Table { tbl }
        }
    }

}

pub mod render {
    use crate::{diff_table, CelAut, CellValue, CELAUT_SIZE};

    pub trait Target {
        fn set_value(&mut self, x: u32, y: u32, val: CellValue);
    }

    pub fn render_evolution(
        celaut: &mut CelAut,
        tbl: &diff_table::Table,
        target: &mut impl Target,
    ) {
        let rule = |l, c, r| tbl.lookup(l, c, r);
        for y in 0..CELAUT_SIZE {
            for x in 0..CELAUT_SIZE {
                target.set_value(x as u32, y as u32, celaut.universe[x]);
            }
            celaut.advance(rule);
        }
    }

}
