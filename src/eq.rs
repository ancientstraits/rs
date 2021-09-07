pub enum Equation {
    // y = mx + b
    SlopeIntercept(f32, f32),

    // ax + by = c
    Standard(f32, f32, f32),
}

impl Equation {
    pub fn to_other(&self) -> Equation {
        match self {
            &Self::SlopeIntercept(m, b) =>
                => Equation::Standard(0.0, 0.0, 0,0)
            &Self::Standard(a, b, c)
                => Equation::SlopeIntercept((a * -1.0) / b, c / b),
        }
    }
}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::SlopeIntercept(m, b) => {
                    format!("y = {}x + {}", m, b)
                }
                Self::Standard(a, b, c) => {
                    format!("{}x + {}y = {}", a, b, c)
                }
            }
        )
    }
}

enum Solution {
    Point(f32, f32),
    None,
    All,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Solution::None => "No Solution".to_string(),
                Solution::All => "All Solutions".to_string(),
                Solution::Point(x, y) => format!("({}, {})", x, y),
            }
        )
    }
}

pub fn find_x(ea: Equation, eb: Equation) -> Solution {
    match (&ea, &eb) {
        (&Equation::SlopeIntercept(a, b), &Equation::SlopeIntercept(c, d)) => {
            if a == c {
                if b == d {
                    return Solution::All;
                } else {
                    return Solution::None;
                }
            } else {
                let x: f32 = (d - b) / (a - c);
                return Solution::Point(x, (a * x) + b);
            }
        }
        (&Equation::Standard(a, b, c), &Equation::SlopeIntercept(d, e)) |
        (&Equation::SlopeIntercept(d, e), &Equation::Standard(a, b, c)) => {
            match Equation::Standard(a, b, c).to_other() {
                Equation::SlopeIntercept(d, e) => Solution::All,
                Equation::SlopeIntercept(d, _) => Solution::None
            }
        }
            // (c - (b * e)) / (a + (b * d)),
        (&Equation::Standard(_, _, _), &Equation::Standard(_, _, _)) =>
            find_x(ea, eb.to_standard().unwrap())
    }
}