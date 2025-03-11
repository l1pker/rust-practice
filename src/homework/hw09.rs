fn rotate(s: String, n: isize) -> String {
    let len = s.len();

    let shift = ((n % len as isize) + len as isize) as usize % len;
    let chars: Vec<char> = s.chars().collect();

    let rotated: Vec<char> = chars[shift..].iter().chain(&chars[..shift]).copied().collect();
    rotated.into_iter().collect()
}
#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.clone(), *n as isize),
                exp.to_string()
            )
        );
}
