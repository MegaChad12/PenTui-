pub enum ManagmentMode {
    Auto,
    //     min max
    //      v   v
    Manual(u16,u16),
}
#[derive(PartialEq, Eq)]
pub enum LineFillMode {
    Center,
    //  the opposite
    //  side margin
    //     v
    Right(usize),
    Left(usize),
}
#[derive(PartialEq, Eq)]
pub enum LayerFillMode {
    Up(usize),
    Down(usize),
    Center,
}
#[derive(Clone, Copy)]

pub enum Color {
    None,
    Red,
    Green,
    Blue,
    Dark,
    Yellow,
    Orange,
    Purple,
    Grey,
    Cyan,
    Rgb(u8,u8,u8),
}

#[derive(Clone, Copy)]
pub enum Style {
    Bold,
    Italic,
    Normal,
    UnderLined,
    Strike,
}

#[derive(Clone)]
pub enum Container<T> {
    Ref(T),

}
impl<T> Container<T> {
    pub fn get_value(&self) -> &T{
        match self {
            Container::Ref(a) => return a,
        }
    }
    pub fn get_mut_value(&mut self) -> &mut T{
        match self {
            Container::Ref(a) => return a,
        }
    }
    pub fn get_ownership(self) -> T{
        match self {
            Container::Ref(a) => return a,
        }
    }
}