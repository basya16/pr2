use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg_weight = rng.gen_range(10..100);
    vec![avg_weight; n]
}

fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total_weight % n != 0 {
        return -1;
    }

    let average = total_weight / n;
    let mut moves = 0;
    let mut imbalance = 0;

    for &weight in shipments {
        imbalance += (weight as i32 - average as i32) as isize;
        moves += imbalance.abs();
    }

    moves as isize
}

fn display_results(shipments: &Vec<u32>) {
    println!("Shipments: {:?}", shipments);

    let result = count_permutation(shipments);

    if result == -1 {
        println!("It is impossible to balance the shipments evenly.");
    } else {
        println!("Minimum moves required to balance: {}", result);
    }
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];
    let generated_shipments = gen_shipments(5);

    println!("Example 1:");
    display_results(&shipments1);
    
    println!("\nExample 2:");
    display_results(&shipments2);

    println!("\nGenerated Shipments:");
    display_results(&generated_shipments);
}
