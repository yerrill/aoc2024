#[derive(Hash)]
pub struct Point(pub usize, pub usize);

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pt").field(&self.0).field(&self.1).finish()
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

//impl std::hash::Hash for Point {
//    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//        self.0.hash(state);
//        self.1.hash(state);
//    }
//}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<(usize, usize)> for Point {
    type Output = Point;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        let Ok(self_x) = isize::try_from(self.0) else {
            return None;
        };
        let Ok(self_y) = isize::try_from(self.1) else {
            return None;
        };

        let Ok(x) = usize::try_from(self_x + rhs.0) else {
            return None;
        };
        let Ok(y) = usize::try_from(self_y + rhs.1) else {
            return None;
        };

        Some(Point(x, y))
    }
}

impl Point {
    pub fn move_dir(&self, dir: Dir) -> Option<Point> {
        let d = dir.resolve();
        *self + d
    }

    pub fn in_bounds(&self, x_max: usize, y_max: usize) -> Option<Point> {
        if self.0 < x_max && self.1 < y_max {
            Some(*self)
        } else {
            None
        }
    }
}

pub const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Up => "^",
            Self::Down => "v",
            Self::Left => "<",
            Self::Right => ">",
        })
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for Dir {}

impl Copy for Dir {}

impl Clone for Dir {
    fn clone(&self) -> Self {
        match self {
            Self::Up => Self::Up,
            Self::Down => Self::Down,
            Self::Left => Self::Left,
            Self::Right => Self::Right,
        }
    }
}

impl std::hash::Hash for Dir {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Dir {
    pub fn resolve(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    pub fn difference(&self, other: Self) -> usize {
        let (r1, c1) = self.resolve();
        let (r2, c2) = other.resolve();

        (r2 - r1).abs().max((c2 - c1).abs()) as usize
    }
}

#[derive(Debug)]
struct Cost<T: Eq + PartialEq + std::hash::Hash>(usize, T);

impl<T: Eq + PartialEq + std::hash::Hash> PartialEq for Cost<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T: Eq + PartialEq + std::hash::Hash> Eq for Cost<T> {}

impl<T: Eq + PartialEq + std::hash::Hash> Ord for Cost<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Eq + PartialEq + std::hash::Hash> PartialOrd for Cost<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + PartialEq + std::hash::Hash> std::hash::Hash for Cost<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Debug)]
pub struct MinHeap<T: Eq + PartialEq + std::hash::Hash> {
    bh: std::collections::BinaryHeap<Cost<T>>,
}

impl<T: Eq + PartialEq + std::hash::Hash> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap {
            bh: std::collections::BinaryHeap::<Cost<T>>::new(),
        }
    }

    pub fn push(&mut self, cost: usize, item: T) {
        self.bh.push(Cost(cost, item));
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if let Some(Cost(c, i)) = self.bh.pop() {
            Some((c, i))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.bh.len()
    }
}

pub struct Board<T: std::fmt::Debug, const R: usize, const C: usize> {
    pub arr: [[T; C]; R],
}

impl<T: std::fmt::Debug, const R: usize, const C: usize> std::fmt::Debug for Board<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        out += "\n";

        for row in 0..R {
            for col in 0..C {
                out += format!("{:?}", self.arr[row][col]).as_str();
            }

            out += "\n";
        }
        f.write_str(out.as_str())
    }
}

impl<T: std::fmt::Debug, const R: usize, const C: usize> Board<T, R, C> {
    pub fn string_using(&self, f: fn(&T) -> String) -> String {
        //&dyn Fn(&T) -> &'static str
        let mut out: String = String::new();
        out += "\n";

        for row in 0..R {
            for col in 0..C {
                out += f(&self.arr[row][col]).as_str();
            }

            out += "\n";
        }
        out
    }

    fn is_valid_point(&self, p: Point) -> bool {
        p.0 < R && p.1 < C
    }

    pub fn translate_point(&self, from: Point, delta: (isize, isize)) -> Option<Point> {
        let new_point = (from + delta)?;

        if self.is_valid_point(new_point) {
            Some(new_point)
        } else {
            None
        }
    }

    pub fn at(&self, p: Point) -> Option<&T> {
        let p = self.translate_point(p, (0, 0))?;

        Some(&self.arr[p.0][p.1])
    }

    pub fn set(&mut self, p: Point, val: T) -> bool {
        let Some(p) = self.translate_point(p, (0, 0)) else {
            return false;
        };
        self.arr[p.0][p.1] = val;
        true
    }

    pub fn dijkstras(
        &self,
        start_cost: usize,
        start_point: Point,
        end_point: Point,
        valid_point: fn(&T) -> bool,
    ) -> (usize, Board<Option<usize>, R, C>) {
        let mut cache: [[Option<usize>; C]; R] = [[None; C]; R];
        let mut pq: MinHeap<Point> = MinHeap::new();
        let mut last_cost = 0;

        pq.push(start_cost, start_point);

        while let Some((current_cost, current_point)) = pq.pop() {
            if cache[current_point.0][current_point.1].is_some() {
                continue;
            } else {
                cache[current_point.0][current_point.1] = Some(current_cost);
            }

            last_cost = current_cost;

            if current_point == end_point {
                break;
            }

            for dir in DIRS {
                let Some(new_point) = self.translate_point(current_point, dir.resolve()) else {
                    continue;
                };
                
                let Some(val_new_point) = self.at(new_point) else {
                    continue;
                };

                if !valid_point(val_new_point) {
                    continue;
                }

                let new_cost = current_cost + 1;

                pq.push(new_cost, new_point);
            }
        }

        (last_cost, Board { arr: cache })
    }
}
