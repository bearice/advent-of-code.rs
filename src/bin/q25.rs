fn loopsize(target: usize) -> usize {
    let mut ret = 0;
    let mut val = 1;
    while val != target {
        val *= 7;
        val %= 20201227;
        ret += 1;
    }
    return ret;
}

fn transform(n: usize, pubkey: usize) -> usize {
    let mut val = 1;
    for _i in 0..n {
        val *= pubkey;
        val %= 20201227;
    }
    val
}
fn main() {
    let n = loopsize(10441485);
    println!("{}", n);
    let m = transform(n, 1004920);
    println!("{}", m);
}
