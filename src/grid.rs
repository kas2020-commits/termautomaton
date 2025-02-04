use crate::windows::MooreNeighborhood;

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

    pub fn window_moore_wrap(&self, x: usize, y: usize) -> MooreNeighborhood<T> {
        // modulo operator enables periodic boundaries i.e. wrapping
        let xm1 = ((x as i16) - 1).rem_euclid(self.width as i16) as usize;
        let xp1 = ((x as i16) + 1).rem_euclid(self.width as i16) as usize;
        let ym1 = ((y as i16) - 1).rem_euclid(self.height as i16) as usize;
        let yp1 = ((y as i16) + 1).rem_euclid(self.height as i16) as usize;
        MooreNeighborhood {
            top_left: &self.data[self.idx(xm1, ym1)],
            top: &self.data[self.idx(x, ym1)],
            top_right: &self.data[self.idx(xp1, ym1)],
            left: &self.data[self.idx(xm1, y)],
            center: &self.data[self.idx(x, y)],
            right: &self.data[self.idx(xp1, y)],
            bot_left: &self.data[self.idx(xm1, yp1)],
            bot: &self.data[self.idx(x, yp1)],
            bot_right: &self.data[self.idx(xp1, yp1)],
        }
    }

    pub fn window_moore_nowrap(&self, x: usize, y: usize) -> MooreNeighborhood<T> {
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

    pub fn iter<F: Fn(&Self, usize, usize) -> MooreNeighborhood<T>>(
        &self,
        gen_window: F,
    ) -> impl Iterator<Item = (usize, usize, MooreNeighborhood<T>)> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| (x, y)))
            .map(move |(x, y)| (x, y, gen_window(&self, x, y)))
    }
}
