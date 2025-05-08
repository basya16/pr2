fn rotate(s: String, n: isize) -> String {
  let len = s.len();
  if len == 0 {
    return s;
  }
  let shift = ((n % len as isize) + len as isize) % len as isize;
  format!("{}{}", &s[len - shift as usize..], &s[..len - shift as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test() {
   let s = "abcdefgh";
   let shifts = [
       (0,  "abcdefgh"),
       (8,  "abcdefgh"),
       (-8, "abcdefgh"),
       (1,  "habcdefg"),
       (2,  "ghabcdef"),
       (10, "ghabcdef"),
       (-1, "bcdefgha"),
       (-2, "cdefghab"),
       (-10,"cdefghab"),
   ];


   shifts
       .iter()
       .for_each(|(n, exp)|
           assert_eq!(
               rotate2(s, n), 
               exp.to_string()
           )
       );
}

  
