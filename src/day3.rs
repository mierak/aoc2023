use anyhow::Result;

#[derive(Debug)]
enum Mode {
    FindParts,
    GearRatio,
}

pub fn part1(input: &str) -> Result<i32> {
    let mut res: Vec<Vec<Element>> = input
        .lines()
        .map(|line| line.chars().map(Element::new).collect())
        .collect();

    let mut sum = 0;
    for y in 0..res.len() {
        for x in 0..res.first().unwrap().len() {
            if res[y][x].is_symbol() {
                if let Some(val) = sum_adjacents(&mut res, Point { x, y }, Mode::FindParts) {
                    sum += val;
                }
            }
        }
    }

    Ok(sum)
}

pub fn part2(input: &str) -> Result<i32> {
    let mut res: Vec<Vec<Element>> = input
        .lines()
        .map(|line| line.chars().map(Element::new).collect())
        .collect();

    let mut sum = 0;
    for y in 0..res.len() {
        for x in 0..res.first().unwrap().len() {
            if res[y][x].value == '*' {
                if let Some(val) = sum_adjacents(&mut res, Point { x, y }, Mode::GearRatio) {
                    sum += val;
                }
            }
        }
    }

    Ok(sum)
}

fn sum_adjacents(input: &mut Vec<Vec<Element>>, p: Point, mode: Mode) -> Option<i32> {
    let mut sum = 0;
    let mut product = 1;
    let mut adjacents = 0;

    let above = p.above();
    if p.y > 0 && input[above.y][above.x].is_numeric() {
        if let Some(num) = above.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let right = p.right();
    if p.x < input.first().unwrap().len() - 1 && input[right.y][right.x].is_numeric() {
        if let Some(num) = right.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let below = p.below();
    if p.y < input.len() - 1 && input[below.y][below.x].is_numeric() {
        if let Some(num) = below.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let left = p.left();
    if p.x > 0 && input[left.y][left.x].is_numeric() {
        if let Some(num) = left.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let top_right = p.top_right();
    if p.x < input.first().unwrap().len() - 1 && p.y > 0 && input[top_right.y][top_right.x].is_numeric() {
        if let Some(num) = top_right.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let top_left = p.top_left();
    if p.x > 0 && p.y > 0 && input[top_left.y][top_left.x].is_numeric() {
        if let Some(num) = top_left.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let below_right = p.below_right();
    if p.y < input.len() - 1
        && p.x < input.first().unwrap().len() - 1
        && input[below_right.y][below_right.x].is_numeric()
    {
        if let Some(num) = below_right.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    let below_left = p.below_left();
    if p.y < input.len() - 1 && p.x > 0 && input[below_left.y][below_left.x].is_numeric() {
        if let Some(num) = below_left.combine_with_adjacent_into_number(input) {
            adjacents += 1;
            sum += num;
            product *= num;
        }
    }
    match mode {
        Mode::FindParts => Some(sum),
        Mode::GearRatio if adjacents != 2 => None,
        Mode::GearRatio => Some(product),
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn above(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn top_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    fn top_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    fn below_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    fn below_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn combine_with_adjacent_into_number(&self, input: &mut [Vec<Element>]) -> Option<i32> {
        let mut x = self.x;
        let y = self.y;
        let mut res = Vec::new();
        let mut was_already_handled = false;

        while x != 0 && input[y][x].is_numeric() {
            if input[y][x - 1].is_numeric() {
                if input[y][x - 1].handled {
                    was_already_handled = true;
                }
                input[y][x - 1].handled = true;
                res.push(input[y][x - 1].value);
            }
            x -= 1;
        }
        res.reverse();
        let mut x = self.x;
        res.push(input[self.y][self.x].value);
        input[y][x].handled = true;
        while x < input.first().unwrap().len() - 1 && input[y][x].is_numeric() {
            if input[y][x + 1].is_numeric() {
                if input[y][x + 1].handled {
                    was_already_handled = true;
                }
                input[y][x + 1].handled = true;
                res.push(input[y][x + 1].value);
            }
            x += 1;
        }

        if was_already_handled {
            return None;
        }

        Some(res.iter().collect::<String>().parse::<i32>().unwrap())
    }
}

#[derive(Debug)]
struct Element {
    handled: bool,
    value: char,
}

impl Element {
    fn new(value: char) -> Self {
        Self { handled: false, value }
    }

    fn is_numeric(&self) -> bool {
        self.value.is_numeric()
    }

    fn is_symbol(&self) -> bool {
        self.value != '.' && !self.value.is_numeric()
    }
}
