#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    const VALUES: [Self; 4] = [Self::Top, Self::Bottom, Self::Left, Self::Right];
}

enum Error {
    SumXYTooMuch,
}

#[derive(Debug, Default)]
pub struct Ant {
    pos: Position,
}

impl Ant {
    pub fn new(position: Position) -> Self {
        Self { pos: position }
    }

    pub fn research(self) -> usize {
        let mut steps = Vec::from([self.pos]);
        let mut step_index: usize = 0;

        while step_index < steps.len() {
            let mut ant = Ant::new(steps[step_index]);

            for direction in Direction::VALUES {
                if let Ok(pos) = ant.step(direction) {
                    if steps.contains(&pos) == false {
                        steps.push(pos);
                    }
                }
            }

            step_index = step_index + 1;
        }

        steps.len()
    }

    fn step(&mut self, direction: Direction) -> Result<Position, Error> {
        let mut new_pos = self.pos.clone();

        match direction {
            Direction::Top => new_pos.y = new_pos.y + 1,
            Direction::Bottom => new_pos.y = new_pos.y - 1,
            Direction::Right => new_pos.x = new_pos.x + 1,
            Direction::Left => new_pos.x = new_pos.x - 1,
        };

        self.validate_new_position(new_pos)?;

        Ok(new_pos)
    }

    fn validate_new_position(&self, new_pos: Position) -> Result<(), Error> {
        let x_sum: u32 = new_pos.x.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
        let y_sum: u32 = new_pos.y.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();

        if x_sum + y_sum > 25 {
            return Err(Error::SumXYTooMuch);
        }

        Ok(())
    }
}
