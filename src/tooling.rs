#[macro_export]
macro_rules! impl_from_int {
    ($ty:ty) => {
        impl From<$ty> for Num {
            fn from(value: $ty) -> Self {
                let postivity = value.is_positive();
                let value = value.abs();
                let array = value.to_le_bytes();
                Self {
                    data: array.to_vec(),
                    polarity: postivity,
                }
            }
        }

        impl From<&$ty> for Num {
            fn from(value: &$ty) -> Self {
                let postivity = value.is_positive();
                let value = value.abs();
                let array = value.to_le_bytes();
                Self {
                    data: array.to_vec(),
                    polarity: postivity,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_u {
    ($ty:ty) => {
        impl From<$ty> for Num {
            fn from(value: $ty) -> Self {
                let array = value.to_le_bytes();
                Self {
                    data: array.to_vec(),
                    polarity: true,
                }
            }
        }

        impl From<&$ty> for Num {
            fn from(value: &$ty) -> Self {
                let array = value.to_le_bytes();
                Self {
                    data: array.to_vec(),
                    polarity: true,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_int {
    ($ty:ty) => {
        impl TryFrom<Num> for $ty {
            type Error = Errors;
        
            fn try_from(value: Num) -> Result<Self, Self::Error> {
                let mut data = value.get_data().to_vec();
                let polarity = if value.polarity { 1 } else { -1 };
                let size = std::mem::size_of::<Self>();
                if data.len() > size {
                    Err(Errors::ToBig)
                } else {
                    if data.len() < size {
                        let error = size - data.len();
                        data.append(&mut vec![0; error]);
                    }
                    assert_eq!(size, data.len());
                    let bytes = data.try_into().unwrap();
                    Ok(Self::from_le_bytes(bytes) * polarity)
                }
            }
        }

        impl TryFrom<&Num> for $ty {
            type Error = Errors;
        
            fn try_from(value: &Num) -> Result<Self, Self::Error> {
                let mut data = value.get_data().to_vec();
                let polarity = if value.polarity { 1 } else { -1 };
                let size = std::mem::size_of::<Self>();
                if data.len() > size {
                    Err(Errors::ToBig)
                } else {
                    if data.len() < size {
                        let error = size - data.len();
                        data.append(&mut vec![0; error]);
                    }
                    assert_eq!(size, data.len());
                    let bytes = data.try_into().unwrap();
                    Ok(Self::from_le_bytes(bytes) * polarity)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_u {
    ($ty:ty) => {
        impl TryFrom<Num> for $ty {
            type Error = Errors;
        
            fn try_from(value: Num) -> Result<Self, Self::Error> {
                let mut data = value.get_data().to_vec();
                let size = std::mem::size_of::<Self>();
                if data.len() > size {
                    Err(Errors::ToBig)
                } else {
                    if data.len() < size {
                        let error = size - data.len();
                        data.append(&mut vec![0; error]);
                    }
                    assert_eq!(size, data.len());
                    let bytes = data.try_into().unwrap();
                    Ok(Self::from_le_bytes(bytes))
                }
            }
        }

        impl TryFrom<&Num> for $ty {
            type Error = Errors;
        
            fn try_from(value: &Num) -> Result<Self, Self::Error> {
                let mut data = value.get_data().to_vec();
                let size = std::mem::size_of::<Self>();
                if data.len() > size {
                    Err(Errors::ToBig)
                } else {
                    if data.len() < size {
                        let error = size - data.len();
                        data.append(&mut vec![0; error]);
                    }
                    assert_eq!(size, data.len());
                    let bytes = data.try_into().unwrap();
                    Ok(Self::from_le_bytes(bytes))
                }
            }
        }
    };
}