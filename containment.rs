fn is_contained<'a>(a: &'a str, b: &'a str) -> bool {
    if a.len() > b.len() {
        return false;
    }

    for i in 0..(b.len() - a.len()) {
        let mut c: usize = 0;
        for j in 0..a.len() {
            if a.as_bytes()[j] == b.as_bytes()[j+i] {
                c += 1;
                if c == x.len() {
                    return true;
                }
            }
        }
    }
    false
}

fn check_containment(inp: &Vec<&str>) -> &str {
    let mut containment: HashMap<usize, usize> = HashMap::new();
    let highest = 0;

    for i in 0..inp.len() {
        let mut score = 0;
        let mut seen = 0;
        for j in 0..inp.len() {
            let a = inp[i];
            let b = inp[j];
            if j == i {
                score += 1;
                seen = (seen + 1) << j;
            }
            else if ((1 << j) & seen) == 0 {
                if is_contained(a, b) {
                    seen = (seen + 1) << j;
                }
            }
        }
    }
}

fn update_score(score: u64, data: usize, input_size: usize, seen: usize, containment: &HashMap<usize, usize>) -> u64 {
    // Finish dynamic programming (upward tree traversal, no loops!)
    // todo: finish function
    for i in 0..input_size {
        if ((1 << i) & data) == 1 {
            score += 1;
            score = update_score(score, containment.get());
            // to-finish the algorithm
        }
    }
}


struct Resultant <'a> {
    data: &'a str,
    message: &'a str
}

fn main() {}
