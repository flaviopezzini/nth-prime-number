pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut current_number = 3;

    let target_size: usize = (n + 1) as usize; // n is 0-indexed

    let mut prime_vec = vec![2];

    loop {
        if is_prime(current_number, &prime_vec) {
            prime_vec.push(current_number);
            
            if prime_vec.len() == target_size {
                break;
            }
        }
        if current_number == u32::MAX {
            panic!("The value of 'n' provided would result in a number that can't fit in a u32 variable");
        }
        current_number += 2;
    }

    current_number
}

fn is_prime(number: u32, prime_vec: &Vec<u32>) -> bool {
    if number <= 1 {
        return false;
    } else {
        for i in 0..prime_vec.len() {
            let current_prime = prime_vec[i] as f32;
            let division:f32 = number as f32 / current_prime;
            if division.fract() == 0.0 {
                return false;
            }
            if current_prime > division {
                return true;
            }
        }

        true
    }
}
