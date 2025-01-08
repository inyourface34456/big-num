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
        let mut new_dat = vec![];
        for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
            new_dat.push(dat | other_dat)
        }
        Self {
            data: new_dat,
            polarity: self.polarity,
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