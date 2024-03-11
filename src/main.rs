fn main() {
    println!("Eratosztheneszi szita");

    let array: [i32; 100] = [0; 100];
    for i in 2..100 {
        let num: i32 = i;
        if array[i as usize] % num==1 {
            println!("{} ", i);
        }
    }
}
