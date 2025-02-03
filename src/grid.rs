use crate::window::MooreNeighborhood;

#[derive(Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub default: T,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: PartialEq + Copy,
{
    pub fn new(data: Vec<T>, width: usize, height: usize, default: T) -> Self {
        assert_eq!(data.len(), width * height, "Invalid grid dimensions");
        Grid {
            data,
            width,
            height,
            default,
        }
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn is_valid(&self, x: i16, y: i16) -> bool {
        x >= 0 && x < self.width as i16 && y >= 0 && y < self.height as i16
    }

    pub fn at(&self, x: i16, y: i16) -> &T {
        if !self.is_valid(x, y) {
            &self.default
        } else {
            &self.data[self.idx(x as usize, y as usize)]
        }
    }

    pub fn make_window(&self, x: usize, y: usize) -> MooreNeighborhood<T> {
        MooreNeighborhood {
            top_left: self.at(x as i16 - 1, y as i16 - 1),
            top: self.at(x as i16, y as i16 - 1),
            top_right: self.at(x as i16 + 1, y as i16 - 1),
            left: self.at(x as i16 - 1, y as i16),
            center: self.at(x as i16, y as i16),
            right: self.at(x as i16 + 1, y as i16),
            bot_left: self.at(x as i16 - 1, y as i16 + 1),
            bot: self.at(x as i16, y as i16 + 1),
            bot_right: self.at(x as i16 + 1, y as i16 + 1),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, MooreNeighborhood<T>)> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| (x, y)))
            .map(move |(x, y)| (x, y, self.make_window(x, y)))
    }
}
