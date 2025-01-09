#![feature(test)]
#![feature(bigint_helper_methods)]

mod tooling;
mod from;
mod into;
mod ops;
mod tests;

//use crate::impl_from;

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Num {
    polarity: bool,
    data: Vec<u8>,
}

impl Default for Num {
    fn default() -> Self {
        Self {
            data: vec![0],
            polarity: true,
        }
    }
}

impl Num {
    pub fn new(data: &[u8]) -> Self {
        if data.len() == 0 {
            panic!("must have at least one element")
        }
        Self {
            data: data.to_owned(),
            polarity: true,
        }
    }

    pub fn incrment(&mut self) {
        let old_dat = &mut self.data;
        let mut rollover = 0;

        for i in old_dat.iter_mut() {
            if i == &u8::MAX {
                *i = 0;
                rollover += 1;
            } else {
                *i += 1;
                break;
            }
        }

        if old_dat.len() == rollover {
            (*old_dat).push(1);
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn to_string(&self) -> String {
        unsafe { std::mem::transmute::<&[u8], &str>(self.get_data()) }.to_string()
    }

    pub fn into_num<'a, T>(&'a self) -> Result<T, <T as TryFrom<&'a Num>>::Error> 
    where
        T: std::convert::TryFrom<&'a Num>
     {
        self.try_into()
    }
}
 