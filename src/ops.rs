use crate::Num;
use std::{iter::zip, ops};

// impl Ord for Num {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let mut data = self.get_data().to_vec();
//         let mut rhs = other.get_data().to_vec();

//         if data.len() > rhs.len() {
//             let error = data.len() - rhs.len();
//             rhs.append(&mut vec![0; error]);
//         } else if data.len() < rhs.len() {
//             let error = rhs.len() - data.len();
//             data.append(&mut vec![0; error]);
//         }

//         for (idx, rhs) in rhs.iter().enumerate().rev() {
//             if &data[idx] > rhs {
//                 println!("found grater");
//                 return std::cmp::Ordering::Greater;
//             } else if &data[idx] < rhs {
//                 println!("found less");
//                 return std::cmp::Ordering::Less;
//             }
//         }
//         std::cmp::Ordering::Equal 
//     }
// }

// impl ops::Neg for Num {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         Self {
//             data: self.data,
//         }
//     }
// }

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
        let polarity = match self.polarity {
            Some(p) => Some(!p),
            None => None
        };
        
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

        if data.len() > rhs.len() {
            let error = data.len() - rhs.len();
            rhs.append(&mut vec![0; error]);
        } else if data.len() < rhs.len() {
            let error = rhs.len() - data.len();
            data.append(&mut vec![0; error]);
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