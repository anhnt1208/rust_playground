// Write function main that will output the result of finaboration
fn main() {
    let n: u32 = env::args().nth(1).unwrap().parse().unwrap();

    if n == 0 || n == 1 {
        println!("{}", n);
    } else {
        let mut fib_prev = 0;
        let mut fib_curr = 1;
        let mut fib_next = 0;

        for _ in 2..=n {
            fib_next = fib_prev + fib_curr;
            fib_prev = fib_curr;
            fib_curr = fib_next;
        }

        println!("{}", fib_curr);
    }
}

// Write testcases for the main function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finaboration() {
        assert_eq!(finaboration(0), 0);
        assert_eq!(finaboration(1), 1);
        assert_eq!(finaboration(2), 1);
        assert_eq!(finaboration(3), 2);
        assert_eq!(finaboration(4), 3);
        assert_eq!(finaboration(5), 5);
        assert_eq!(finaboration(10), 55);
    }
}