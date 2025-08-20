trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

trait Sleep {
    fn sleep(&self) {
        println!("I'm sleeping!");
    }
}

struct Robot {}
impl Run for Robot {
    fn run(&self) {
        println!("I'm running fast!");
    }
}
impl Sleep for Robot {}

fn main() {
    let robot = Robot {};
    robot.run();
    robot.sleep();
}