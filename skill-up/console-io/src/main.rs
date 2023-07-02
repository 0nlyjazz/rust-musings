#[macro_use]
extern crate scan_rules;

fn main() {
    let result = try_readln! {
        (let m: i32, let n: i32, let o: i32) => (m,n,o)
    };
    match result {
        Ok((n,m,o)) => println!("got data m = {}, n = {}, o = {}", 
                m,n,o),
        Err(e) => println!("failed to parse {}", e),
    };
}
