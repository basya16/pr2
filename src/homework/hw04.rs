const W: u32 = 11;
const H: u32 = 5;
fn main() { 
    for y in 0..H {
      let spaces = H - y - 1;
      let stars = 2 * y + 1;

      for _ in 0..spaces {
            print!(" ");
        }
      for _ in 0..stars {
            println!("*");
        }

      println!();
    }
   for y in (0..H - 1).rev() {
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

    

      


  
