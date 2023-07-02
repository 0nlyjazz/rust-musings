/*
 * read a matrix and print it...
 */

use std::io;
fn display_matrix(m: &Vec<Vec<i32>>) {
    /* print the matrix */
    m.iter().for_each(|it| {
        it.iter().for_each(|element| {
            print!("{} ", element);
        });
        println!(" ");
    })
}
fn main() {
    let mut tc = String::new();
    io::stdin()
        .read_line(&mut tc)
        .expect("Need no. of test cases");

    let itc:usize = tc.trim().parse().expect("invalid test case");
    /* Test case loop */
    for tc_index in 0..itc {
        
        //println!("tc = {}", &tc_index+1);                    
        let mut dim = String::new();
        io::stdin()
            .read_line(&mut dim)
            .expect("need matrix dimension");
        //println!("got dimension = {}", &dim);
        let idim: usize = dim
                    .trim()
                    .parse().expect("Invalid matrix dimension");

        //println!("got dimension = {}", &idim);
        /* alloc the matrix */
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for _ in 0..idim {
            /* read the input */
            let mut a_row = String::new();
            io::stdin()
                .read_line(&mut a_row)
                .expect("Can't find a row of numbers to be read");
            //println!("got row = {}", &a_row);
           
            let p = 
                    a_row.split_whitespace().map(|s| s.parse());
            let rvec: Result<Vec<i32>, _> = p.collect();
            //println!("rvec = {:#?}",&rvec);
            matrix.push(rvec.unwrap());
        }

        println!("#{}", &tc_index+1);
        display_matrix(&matrix);
    }
}
