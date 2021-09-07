use eq::Equation;

pub mod eq;

fn main() {
    let ea = Equation::SlopeIntercept(2.0, 3.0);
    let eb = Equation::SlopeIntercept(3.0, 2.5);
    ea.print();
    eb.print();
    println!("{}", eq::find_x(ea, eb));
}