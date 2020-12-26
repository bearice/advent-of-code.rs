fn step(v: &mut Vec<usize>) {
    let mut max = v.len() - 1;
    let cur = v[0];
    // println!("cur={}", cur);
    let mut dst = cur - 1;
    let mut take = vec![0, 0, 0];
    let mut next = v[cur];
    for t in 0..3 {
        take[t] = next;
        next = v[next];
    }
    v[cur] = next;
    // rem -= 4;
    while take.contains(&max) {
        max -= 1;
    }
    // j %= l;
    // println!("take={:?}", take);
    while take.contains(&dst) {
        dst -= 1;
        // println!("dst candidate={}", dst);
    }
    if dst == 0 {
        dst = max;
    }
    // println!("dst={}", dst);
    next = dst;
    for t in take {
        let old = v[next];
        v[next] = t;
        v[t] = old;
        next = t;
    }
    v[0] = v[cur];
}

fn dump(temp: &Vec<usize>) {
    let mut hd = temp[0];
    while temp[hd] != temp[0] {
        print!("{}", hd);
        hd = temp[hd];
    }
    println!("{}", hd);
}
fn main() {
    let mut initial = vec![3, 2, 6, 5, 1, 9, 4, 7, 8];
    initial.append(&mut (10..=1_000_000).collect::<Vec<_>>());
    println!("total={}", initial.len());
    let mut pos = 0;
    let mut temp = initial.clone();
    temp.push(0);
    for i in initial {
        temp[pos] = i;
        pos = i;
    }
    temp[pos] = temp[0];

    // println!("temp={}", temp.len());
    // dump(&temp);
    for _i in 0usize..10_000_000 {
        step(&mut temp);
        // dump(&temp);
    }
    // println!("{:?}", temp);
    let n1 = temp[1];
    let n2 = temp[n1];
    println!("{} {} {}", n1, n2, n1 * n2);
}
