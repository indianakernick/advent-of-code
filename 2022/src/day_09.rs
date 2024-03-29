use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, usize) {
    let mut rope: [(i32, i32); 10] = Default::default();
    let mut visits_1 = HashSet::<(i32, i32)>::new();
    let mut visits_9 = HashSet::<(i32, i32)>::new();

    visits_1.insert((0, 0));
    visits_9.insert((0, 0));

    for line in input.lines() {
        let direction = match line.as_bytes()[0] {
            b'U' => ( 0, -1),
            b'R' => ( 1,  0),
            b'D' => ( 0,  1),
            b'L' => (-1,  0),
            _ => panic!("Invalid direction")
        };

        let distance: u32 = line[2..].parse().unwrap();

        for _ in 0..distance {
            rope[0].0 += direction.0;
            rope[0].1 += direction.1;

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                let tail_to_head = (head.0 - tail.0, head.1 - tail.1);

                if tail_to_head.0.abs() < 2 && tail_to_head.1.abs() < 2 {
                    continue;
                }

                tail.0 += tail_to_head.0.signum();
                tail.1 += tail_to_head.1.signum();
            }

            visits_1.insert(rope[1]);
            visits_9.insert(rope[9]);
        }
    }

    (visits_1.len(), visits_9.len())
}

#[cfg(test)]
#[test]
fn example_1() {
    let input =
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let output = solve(input);
    assert_eq!(output.0, 13);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input =
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let output = solve(input);
    assert_eq!(output.1, 36);
}
