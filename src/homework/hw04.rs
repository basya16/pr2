const W: u32 = 11;
const H: u32 = 5;

fn main() { 
    let total_rows = 2 * H - 1;

    for i in 0..total_rows {
        let y = if i < H {
            i
        } else {
            total_rows - i - 1
        };
        
        let spaces = H - y - 1;
        let stars = 2 * y + 1;

        for _ in 0..spaces {
            print!(" ");
        }

        for _ in 0..stars {
            print!("*");
        }

      
        println!();
    }
}

    

      


  
