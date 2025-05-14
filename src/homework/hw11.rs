use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
  let mut min_sum = i32::MAX;
  let mut min_index = 0;

  for i in 0..data.len() - 1 {
    let sum = data[i] + data[i + 1];
    if sum < min_sum {
      min_sum = sum;
      min_index = i;
    }
  }
  (min_sum, min_index, min_index + 1)
}

fn display_vector(data: &[i32]) {
   println!("indexes: {}", (0..data.len()).map(|x| format!("{:>3}.", x)).collect::<Vec<String>>().join(" "));
  
   println!("data:   [{}]", data.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));

   let (min_sum, idx1, idx2) = min_adjacent_sum(data);

  let mut arrows = vec!["   "; data.len()];
    arrows[idx1] = "\\__";
    arrows[idx2] = "__/";

    println!("indexes: {}", arrows.join(" "));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
  let data = gen_random_vector(20);
    display_vector(&data);
}
