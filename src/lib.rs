#[derive(Clone, Copy, Debug)]
pub enum CellValue {
    Zero,
    One,
    Two,
    Three,
}

pub const CELL_LIMIT: usize = 4;
pub const TABLE_SIZE: usize = 2 * CELL_LIMIT - 1;

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
    pub fn new(universe: Universe) -> Self {
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

    pub struct Table {
        tbl: [[CellValue; TABLE_SIZE]; TABLE_SIZE],
    }

    impl Table {
        pub fn new(tbl: [[CellValue; TABLE_SIZE]; TABLE_SIZE]) -> Self {
            Table { tbl }
        }

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

        pub fn value_at(&self, i: usize, j: usize) -> CellValue {
            self.tbl[i][j]
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

pub mod convert {
    use crate::diff_table::Table;
    use crate::{CellValue, Universe, CELAUT_SIZE, TABLE_SIZE};

    fn acceptable_char(c: char) -> bool {
        c == '0' || c == '1' || c == '2' || c == '3'
    }

    fn cell_value_from_char(c: char) -> CellValue {
        match c {
            '0' => CellValue::Zero,
            '1' => CellValue::One,
            '2' => CellValue::Two,
            '3' => CellValue::Three,
            _ => panic!("cell_value_from_char called with invalid character"),
        }
    }

    pub fn table_from_str(src: &str) -> Result<Table, &'static str> {
        if src.len() != 49 {
            return Err("Expected exactly 25 characters");
        };
        if src.chars().any(|c| !acceptable_char(c)) {
            return Err("Unexpected characters: only 0, 1, 2, & 3 are allowed");
        };
        let src_values: Vec<CellValue> = src.chars().map(cell_value_from_char).collect();
        let mut tbl = [[CellValue::Zero; TABLE_SIZE]; TABLE_SIZE];
        for i in 0..TABLE_SIZE {
            for j in 0..TABLE_SIZE {
                tbl[i][j] = src_values[j * TABLE_SIZE + i];
            }
        }
        Ok(Table::new(tbl))
    }

    #[test]
    fn test_table_from_str() {
        let src = "0223302003331222121130103120000103211332113202133";
        table_from_str(src).unwrap();
    }

    pub fn table_to_string(tbl: &Table) -> String {
        let mut result = String::new();
        for j in 0..TABLE_SIZE {
            for i in 0..TABLE_SIZE {
                let v = tbl.value_at(i, j);
                let c = cellvalue_to_char(v);
                result.push(c);
            }
        }
        result
    }

    fn cellvalue_to_char(v: CellValue) -> char {
        match v {
            CellValue::Zero => '0',
            CellValue::One => '1',
            CellValue::Two => '2',
            CellValue::Three => '3',
        }
    }

    #[test]
    fn test_table_roundrtip() {
        {
            let src = "0223302003331222121130103120000103211332113202133";
            let tbl = table_from_str(src).unwrap();
            let encoded = table_to_string(&tbl);
            assert_eq!(src, &encoded);
        }
    }

    pub fn universe_from_str(src: &str) -> Result<Universe, &'static str> {
        if src.len() != CELAUT_SIZE {
            return Err("Expected exactly 128 characters");
        }
        let mut universe = [CellValue::Zero; CELAUT_SIZE];
        for (idx, cellval) in src.chars().map(cell_value_from_char).enumerate() {
            universe[idx] = cellval;
        }
        Ok(universe)
    }

    #[test]
    fn test_universe_from_str() {
        let src = "13322312112013303223211001320220302102013331230322130200133020200210112200303020332111213131303221201233021000210131000330110130";
        universe_from_str(src).unwrap();
    }
}
