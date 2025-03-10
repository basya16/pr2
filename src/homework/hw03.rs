fn main() {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 10;

    for i in 0..WIDTH {
        print!("*");
    }
    println!();

    for _ in 0..HEIGHT - 2 {
        print!("*");
        for _ in 0..WIDTH - 2 {
            print!(" ");
        }
        println!("*");
    }

    for i in 0..WIDTH {
        print!("*");
    }
    println!();
}

