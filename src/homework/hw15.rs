fn solve() -> Vec<([u8; 4], u8, [u8; 4])> {
    let mut solutions = Vec::new();

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m {
                continue;
            }
            for x in 1..=8 {
                if x == m || x == u {
                    continue;
                }
                for a in 1..=8 {
                    if a == m || a == u || a == x {
                        continue;
                    }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a {
                            continue;
                        }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s {
                                continue;
                            }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l {
                                    continue;
                                }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o {
                                        continue;
                                    }

                                    let muxa = (m as u32) * 1000 + (u as u32) * 100 + (x as u32) * 10 + (a as u32);
                                    let slon = (s as u32) * 1000 + (l as u32) * 100 + (o as u32) * 10 + (n as u32);

                                    if muxa * (a as u32) == slon {
                                        solutions.push(([m, u, x, a], a, [s, l, o, n]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

fn main() {
    let solutions = solve();

    println!("Знайдено {} розв'язків:", solutions.len());

    for (muxa_arr, a, slon_arr) in &solutions {
        let m = muxa_arr[0];
        let u = muxa_arr[1];
        let x = muxa_arr[2];
        let a_muxa = muxa_arr[3];
        let s = slon_arr[0];
        let l = slon_arr[1];
        let o = slon_arr[2];
        let n = slon_arr[3];

        println!("  muxa: {} {} {} {}", m, u, x, a_muxa);
        println!("x    a:    {}", a);
        println!("-------");
        println!("  slon: {} {} {} {}", s, l, o, n);
        println!();
    }
}
