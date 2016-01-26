fn main() {
    let mut children = vec![];
    children.push(thread::spawn(move || {10*2}));
    children.push(thread::spawn(move || {20*2}));
    children.push(thread::spawn(move || {30+40}));

    let adder = thread::spawn(move || {
        let mut sum = 0;
        let result = children.into_iter().map(|child| {
            let value = child.join().unwrap();
            sum += value;
            format!("{}", value)
        }).collect::<Vec<_>>().join(" + ");
        return format!("{} = {}", result, sum);
    });

    let result = adder.join().unwrap();
    println!("{}", result);
}