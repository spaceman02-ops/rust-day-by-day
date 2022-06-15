fn main() {
    //first let's make a new vector
    let mut v = vec![];
    //now let's add some values
    for x in 0..30 {
        print!("{} ", x);
        v.push(x);
    }
    //let's try mapping
    let mult: Vec<i32> = v.iter().map(|x| x * 2).collect();
    //let's try filtering
    let filt = v.iter().filter(|y| *y % 2 == 0);
    println!("mapped");
    for o in &mult {
        print!("{} ", o)
    }
    println!("filtered");
    for p in filt {
        print!("{} ", p)
    }
}
