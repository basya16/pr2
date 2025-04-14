const W: u32 = 28; //це ширина конверта
const H: u32 = 10; //висота

fn main() {
    for y in 0..H {
        for x in 0..W {
            let border = y == 0 || y == H - 1 || x == 0 || x == W - 1;
            let diag1_x = ((W - 1) as f32 * (y as f32) / (H - 1) as f32).round() as u32;
            let diag2_x = ((W - 1) as f32 * ((H - 1 - y) as f32) / (H - 1) as f32).round() as u32;
            let diagonal = x == diag1_x;
            let diagonal2 = x == diag2_x;
           
            let show =  border || diagonal || diagonal2;
            let sym: &str =  if show { "*" } else { " "};
            print!("{}", sym);
        }
        println!();
    } 
}    
