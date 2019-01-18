use std::{
    env,
    fs::File,
};
use pi_collision::Collisions;


fn main() {
    let arg = env::args().nth(1).and_then(|s| s.parse::<f64>().ok());
    let mass_bigger = if let Some(mass) = arg {
        mass
    } else {
        eprintln!("Mass parameter missing! Usage:");
        eprintln!("    cli <mass-of-bigger-object>");
        eprintln!("");
        eprintln!("Example:");
        eprintln!("    cli 100");
        return;
    };

    let c = Collisions::calculate(mass_bigger);
    println!("Number of collisions: {}", c.count());

    let f = File::create("out.svg").unwrap();
    c.write_svg(f).unwrap();
}
