const W: u32 = 30; //це ширина конверта
const H: u32 = 15; //висота

fn main() {
    for y in 0..H {
        for x in 0..W {
            let border: bool = y == 0 || y == H - 1 || x == 0 || x == W - 1;
            let diagonal: bool = x == 2 * y;
            let diagonal2: bool = x == W - 1 - 2 * y;
            let show: bool =  border || diagonal || diagonal2;
            let sym: &str =  if show { "*" } else { " "};
            print!("{}", sym);
        }
        printl!();
    } 
}    
