use crate::Num;
use std::{iter::zip, ops};

impl Ord for Num {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let own_dat = self.get_data().to_vec();
        let other_dat = other.get_data().to_vec();

        for (dat, other_dat) in zip(own_dat, other_dat) {
            if dat > other_dat {
                return std::cmp::Ordering::Greater;
            } else if dat < other_dat {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal 
    }
}

impl ops::Neg for Num {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: self.data,
            polarity: !self.polarity,
        }
    }
}

impl ops::BitAnd for Num {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new_dat = vec![];
        for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
            new_dat.push(dat & other_dat)
        }
        Self {
            data: new_dat,
            polarity: self.polarity,
        }
    }
}

impl ops::BitAndAssign for Num {
    fn bitand_assign(&mut self, rhs: Self) {
        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat & other_dat;
        }
    }
}

impl ops::BitOr for Num {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new_dat = self.get_data().to_vec();
        for (idx, other_dat) in rhs.get_data().iter().enumerate() {
            new_dat[idx] |= other_dat
        }
        Self {
            data: new_dat,
            polarity: self.polarity/*  | rhs.polarity*/,
        }
    }
}

impl ops::BitOrAssign for Num {
    fn bitor_assign(&mut self, rhs: Self) {
        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat | other_dat;
        }
    }
}

impl ops::BitXor for Num {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new_dat = vec![];
        for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
            new_dat.push(dat ^ other_dat)
        }
        Self {
            data: new_dat,
            polarity: self.polarity,
        }
    }
}

impl ops::BitXorAssign for Num {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat ^ other_dat;
        }
    }
}

impl ops::Not for Num {
    type Output = Num;

    fn not(self) -> Self::Output {
        let mut new_dat = Vec::with_capacity(self.data.len());
        for dat in self.data.iter() {
            new_dat.push(!dat); 
        }

        //dbg!(Self {data: new_dat.clone(), polarity: !self.polarity});
        Self {data: new_dat, polarity: self.polarity}
    }
} 

impl ops::Add for Num {
    type Output = Num;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        let mut rhs = rhs.data;

        if data.len() > rhs.len() {
            let error = data.len() - rhs.len();
            rhs.append(&mut vec![0; error]);
        } else if data.len() < rhs.len() {
            let error = rhs.len() - data.len();
            data.append(&mut vec![0; error]);
        }

        assert!(data.len() == rhs.len());

        let mut carry_flag: u16 = 0;
        let mut set_on_last = false;

        for (idx, rhs) in rhs.into_iter().enumerate() {
            // let add_res = rhs as u16 + data[idx] as u16 + carry_flag;
            // if add_res > 255 as u16 {
            //     carry_flag = (add_res / 256) as u16;
            //     data[idx] = (add_res % 256) as u8;
            //     set_on_last = true;
            // } else {
            //     data[idx] = add_res as u8;
            //     set_on_last = false;
            //     carry_flag = 0;
            // }

            (data[idx], set_on_last) = data[idx].carrying_add(rhs, set_on_last)
        }

        if set_on_last {
            for (idx, data_) in data.clone().iter().rev().enumerate() {
                if data_ == &0 {
                    data.remove(idx);                    
                }
            }
            data.push(1);
        }
        //dbg!(&data);
        Self {data, polarity: true}
    }
}

impl ops::Sub for Num {
    type Output = Num;
    
    fn sub(self, rhs: Self) -> Self::Output {
        if (rhs.polarity && !self.polarity)  || (!rhs.polarity && self.polarity) {
            return self + rhs;
        }

        Self::default()
    }
} 