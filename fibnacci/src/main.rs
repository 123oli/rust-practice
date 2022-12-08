fn main() {
    fn fib(n: i32) -> i32 {
        if n == 0 {
            0;
        } 
        let mut fi_1 = 1;
        let mut fi_2 = 1;
        let i = 2;
        for i in i..n {
            let fi = fi_1 + fi_2;
            fi_2 = fi_1;
            fi_1 = fi;
        }
        fi_1
    }
    println!("{}", fib(10));
}
