fn tree(triangle_count: u32) {
    let max_width = (triangle_count * 2 + (triangle_count - 1) * 2) * 2 - 1;
 (1..=3).for_each(|line| {
        let stars = if line == 3 { 3 } else { 1 }; 
        let spaces = (max_width - stars) / 2; 
        let row = " ".repeat(spaces as usize) + &"*".repeat(stars as usize);
        println!("{}", row);
    });

 (1..triangle_count).for_each(|level| {
        (0..=level + 2).for_each(|line| {
            let stars = line * 2 + 1;  
            let spaces = (max_width - stars) / 2; 
            let row = " ".repeat(spaces as usize) + &"*".repeat(stars as usize);
            println!("{}", row);
        });
    });
}

fn main() {
 let triangle_count = 5;
 tree(triangle_count);
}
