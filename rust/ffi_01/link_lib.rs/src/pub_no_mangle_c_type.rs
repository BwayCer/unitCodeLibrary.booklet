#[repr(u8)]
#[derive(Clone)]
pub enum FruitEnum {
    Apple,
    Blackberry,
    Cherry,
    Fig,
    Orange,
}

#[repr(C)]
pub struct PureDataArgs {
    pub argu_64: f64,
    pub argu_32: i32,
    pub argu_16: u16,
    pub argu_enum: FruitEnum,
}

#[repr(C)]
pub struct PureDataResult {
    pub rtn_64: f64,
    pub rtn_32: i32,
    pub rtn_16: u16,
    pub rtn_enum: FruitEnum,
    pub rtn_yn: bool,
}
