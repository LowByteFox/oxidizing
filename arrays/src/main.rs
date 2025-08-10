fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:#?}", a);

    /* type; size */
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("{:#?}", a);
    println!("{:#?}", months);
    let a = [7; 5]; /* item; length */

    println!("{:#?}", a);
}
