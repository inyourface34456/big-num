use crate::Num;
use std::{iter::zip, ops};
use std::cmp::Ordering;

impl Ord for Num {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // // Treat None polarity as positive
        // let self_polarity = self.polarity.unwrap_or(true);
        // let other_polarity = other.polarity.unwrap_or(true);

        // // Compare polarities first
        // match (self_polarity, other_polarity) {
        //     (false, true) => return Ordering::Less, // Negative < Positive
        //     (true, false) => return Ordering::Greater, // Positive > Negative
        //     (false, false) => {
        //         // Both negative: compare magnitudes, reverse order
        //         return compare_magnitudes(&self.data, &other.data).reverse();
        //     }
        //     (true, true) => {
        //         // Both positive: compare magnitudes
        //         return compare_magnitudes(&self.data, &other.data);
        //     }
        // }
        self.data.cmp(&other.data)
    }
}


impl ops::Neg for Num {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut data = self.get_data().to_vec();
        
        let polarity = match self.polarity {
            Some(p) => {
                data[0] ^= 0b10000000;
                Some(!p)
            },
            None => None
        };

        

        Self {
            data: self.data,
            polarity
        }
    }
}

impl ops::BitAnd for Num {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new_dat = vec![];
        
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() & rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() & true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() & true)
        } else {
            None
        };

        for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
            new_dat.push(dat & other_dat)
        }
        Self {
            data: new_dat,
            polarity
        }
    }
}

impl ops::BitAndAssign for Num {
    fn bitand_assign(&mut self, rhs: Self) {
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() & rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() & true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() & true)
        } else {
            None
        };

        self.polarity = polarity;

        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat & other_dat;
        }
    }
}

impl ops::BitOr for Num {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() | rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() | true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() | true)
        } else {
            None
        };

        let mut new_dat = self.get_data().to_vec();
        for (idx, other_dat) in rhs.get_data().iter().enumerate() {
            new_dat[idx] |= other_dat
        }
        Self {
            data: new_dat,
            polarity
        }
    }
}

impl ops::BitOrAssign for Num {
    fn bitor_assign(&mut self, rhs: Self) {
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() | rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() | true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() | true)
        } else {
            None
        };

        self.polarity = polarity;

        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat | other_dat;
        }
    }
}

impl ops::BitXor for Num {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new_dat = vec![];
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() ^ rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() ^ true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() ^ true)
        } else {
            None
        };

        for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
            new_dat.push(dat ^ other_dat)
        }
        Self {
            data: new_dat,
            polarity
        }
    }
}

impl ops::BitXorAssign for Num {
    fn bitxor_assign(&mut self, rhs: Self) {
        let polarity = if self.polarity.is_some() && rhs.polarity.is_some() {
            Some(self.polarity.unwrap() ^ rhs.polarity.unwrap())
        } else if self.polarity.is_some() {
            Some(self.polarity.unwrap() ^ true)
        } else if rhs.polarity.is_some() {
            Some(rhs.polarity.unwrap() ^ true)
        } else {
            None
        };

        self.polarity = polarity;
        for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
            self.data[idx] = dat ^ other_dat;
        }
    }
}

impl ops::Not for Num {
    type Output = Num;

    fn not(self) -> Self::Output {
        let mut new_dat = Vec::with_capacity(self.data.len());
        let polarity = self.polarity.map(|p| !p);
        
        for dat in self.data.iter() {
            new_dat.push(!dat); 
        }

        //dbg!(Self {data: new_dat.clone(), polarity: !self.polarity});
        Self {
            data: new_dat,
            polarity
        }
    }
} 

// impl ops::Shl for Num {
//     type Output = Num;

//     fn shl(self, rhs: Self) -> Self::Output {
//         let mut new_dat = vec![];
//         for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
//             new_dat.push(dat << other_dat)
//         }
//         Self {
//             data: new_dat,
//         }
//     }
// }

// impl ops::ShlAssign for Num {
//     fn shl_assign(&mut self, rhs: Self) {
//         for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
//             self.data[idx] = dat << other_dat;
//         }
//     }
// }

// impl ops::Shr for Num {
//     type Output = Num;

//     fn shr(self, rhs: Self) -> Self::Output {
//         let mut new_dat = vec![];
//         for (dat, other_dat) in zip(self.get_data(), rhs.get_data()) {
//             new_dat.push(dat >> other_dat)
//         }
//         Self {
//             data: new_dat,
//         }
//     }
// }

// impl ops::ShrAssign for Num {
//     fn shr_assign(&mut self, rhs: Self) {
//         for (idx, (dat, other_dat)) in zip(self.data.clone(), rhs.get_data()).enumerate() {
//             self.data[idx] = dat >> other_dat;
//         }
//     }
// }

impl ops::Add for Num {
    type Output = Num;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        let mut rhs = rhs.data;

        match data.len().cmp(&rhs.len()) {
            std::cmp::Ordering::Less => {
                let error = rhs.len() - data.len();
                data.append(&mut vec![0; error]);
            },
            std::cmp::Ordering::Greater => {
                let error = data.len() - rhs.len();
                rhs.append(&mut vec![0; error]);
            }
            _ => {}
        }

        let mut new_data = vec![0; data.len()];

        assert!(data.len() == rhs.len());

        let mut carry_flag = false;

        for (idx, rhs) in rhs.into_iter().enumerate() {
            (new_data[idx], carry_flag) = data[idx].carrying_add(rhs, carry_flag)
        }

        if carry_flag {
            new_data.push(1);
        }

        let polarity = Some((new_data[0] >> 7) == 0);

        Self {
            data: new_data, 
            polarity
        }
    }
}

impl ops::Sub for Num {
    type Output = Num;
    
    fn sub(self, _rhs: Self) -> Self::Output {
        Self::default()
    }
} 

impl ops::Mul for Num {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        let mut rhs = rhs.data;

        match data.len().cmp(&rhs.len()) {
            std::cmp::Ordering::Less => {
                let error = rhs.len() - data.len();
                data.append(&mut vec![0; error]);
            },
            std::cmp::Ordering::Greater => {
                let error = data.len() - rhs.len();
                rhs.append(&mut vec![0; error]);
            }
            _ => {}
        }

        let mut new_data = vec![0; data.len()];

        assert!(data.len() == rhs.len());

        let mut carry = 0;

        for (idx, rhs) in rhs.into_iter().enumerate() {
            (new_data[idx], carry) = data[idx].carrying_mul(rhs, carry)
        }

        if carry != 0 {
            new_data.push(carry);
        }

        let polarity = Some((new_data[0] >> 7) == 0);

        Self {
            data: new_data, 
            polarity
        }
    }
}