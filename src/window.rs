#[derive(Debug)]
pub struct MooreNeighborhood<'a, T>
where
    T: PartialEq,
{
    pub top_left: &'a T,
    pub top: &'a T,
    pub top_right: &'a T,

    pub left: &'a T,
    pub center: &'a T,
    pub right: &'a T,

    pub bot_left: &'a T,
    pub bot: &'a T,
    pub bot_right: &'a T,
}

fn count_<T: PartialEq>(a: &T, b: &T) -> u8 {
    match a == b {
        false => 0,
        true => 1,
    }
}

impl<'a, T> MooreNeighborhood<'a, T>
where
    T: PartialEq,
{
    pub fn count(&self, val: &T) -> u8 {
        count_(self.top_left, val)
            + count_(self.top, val)
            + count_(self.top_right, val)
            + count_(self.left, val)
            + count_(self.right, val)
            + count_(self.bot_left, val)
            + count_(self.bot, val)
            + count_(self.bot_right, val)
    }

    pub fn count_all(&self, val: &T) -> u8 {
        self.count(val) + count_(self.center, val)
    }
}
