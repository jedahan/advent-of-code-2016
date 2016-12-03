use std::fs;
use std::io::Read;

fn main() {
    let mut position = (0, 0);
    let nesw = [ (0, 1), (1, 0), (0, -1), (-1, 0) ];
    let mut direction = nesw.iter().cycle();
    direction.next(); // start facing north

    let mut file = fs::File::open("input").unwrap();
    let mut path = String::new();
    file.read_to_string(&mut path).unwrap();
    path.pop(); // remove extraneous newline

    let turns: Vec<&str> = path.split(", ").collect();
    let mut visited = vec![(0isize, 0isize)];

    for turn in &turns {
        let mut vector = direction.next();
        if turn[0..1] == "L".to_string() { direction.next(); vector = direction.next(); }

        let distance = turn[1..].parse::<usize>().unwrap();

        for _ in 0..distance {
            position.0 += vector.unwrap().0;
            position.1 += vector.unwrap().1;
            if ! visited.is_empty() {
                if visited.contains(&position) {
                    println!("{:?} visited twice, {} blocks away", position, position.0.abs() + position.1.abs());
                    visited.clear();
                } else {
                    visited.push(position);
                }
            }
        }
    }

    println!("{:?}", position.0.abs() + position.1.abs());
}
