use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = 0;
    let mut arrangement_spec = Vec::new();
    let mut expected_group_sizes = Vec::new();

    for line in common::lines_iter(input) {
        let space = line.iter().position(|ch| *ch == b' ').unwrap();

        arrangement_spec.clear();
        arrangement_spec.push(line[0]);

        // Collapse consecutive operational springs.
        for i in 1..space {
            if line[i - 1] == b'.' && line[i] == b'.' {
                continue;
            }
            arrangement_spec.push(line[i]);
        }

        expected_group_sizes.clear();
        expected_group_sizes.extend(line[space + 1..]
            .split(|ch| *ch == b',')
            .map(common::parse_u32));

        fn inner(spec: &[u8], sizes: &[u32]) -> u32 {
            if sizes.len() == 0 {
                return if spec.iter().all(|b| *b == b'.' || *b == b'?') {
                    // Assume that the remaining unknowns are operational.
                    1
                } else {
                    0
                };
            }

            if (spec.len() == 0 && sizes.len() > 0) || spec.len() < sizes[0] as usize {
                return 0;
            }

            if !spec[0..sizes[0] as usize].iter().all(|b| *b == b'#' || *b == b'?') {
                return if spec[0] == b'.' || spec[0] == b'?' {
                    // Assume that the first spring is operational.
                    inner(&spec[1..], sizes)
                } else {
                    0
                };
            }

            if spec.len() > sizes[0] as usize {
                let next = spec[sizes[0] as usize];

                let first = if next == b'.' || next == b'?' {
                    // Assume that spring after this contiguous block of broken
                    // springs is operational.
                    inner(&spec[sizes[0] as usize + 1..], &sizes[1..])
                } else {
                    0
                };

                let second = if spec[0] == b'?' {
                    // Assume that the first spring is operational.
                    inner(&spec[1..], sizes)
                } else {
                    0
                };

                return first + second;
            }

            inner(&spec[sizes[0] as usize..], &sizes[1..])
        }

        sum += inner(&arrangement_spec, &expected_group_sizes);
    }

    (sum, 0)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let output = solve(input);
    assert_eq!(output.0, 21);
}
