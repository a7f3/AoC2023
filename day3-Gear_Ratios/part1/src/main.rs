fn main() {}
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(y: i32, x: i32) -> Self {
        Self { x, y }
    }
}
fn check_symbol(input: &Vec<&str>, coord: Vec2) -> bool {
    if coord.y as usize > input.len() || coord.y < 0 {
        /* Exit if outside input */
        return false;
    }
    if coord.x as usize > input[0].len() {
        /* Exit if outside input */
        return false;
    }
    let c = input[coord.y as usize]
        .chars()
        .nth(coord.x as usize)
        .unwrap();
    if c.is_ascii_graphic() {
        if c != '.' {
            return true;
        }
    }
    false
}

fn check_surrounds(input: Vec<&str>, coord: Vec2) -> bool {
    if coord.y as usize > input.len() {
        /* Exit if outside input */
        return false;
    }
    if coord.x as usize > input[0].len() {
        /* Exit if outside input */
        return false;
    }
    let around = vec![
        Vec2::new(coord.y - 1, coord.x - 1),
        Vec2::new(coord.y - 1, coord.x),
        Vec2::new(coord.y - 1, coord.x + 1),
        Vec2::new(coord.y, coord.x - 1),
        Vec2::new(coord.y, coord.x + 1),
        Vec2::new(coord.y + 1, coord.x - 1),
        Vec2::new(coord.y + 1, coord.x),
        Vec2::new(coord.y + 1, coord.x + 1),
    ];

    for xy in around {
        if check_symbol(&input, xy) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_num() {}

    #[cfg(test)]
    mod single_num {
        use crate::{check_surrounds, Vec2};

        #[test]
        fn around() {
            let input = vec![
                vec!["*..", ".3.", "..."],
                vec![".*.", ".3.", "..."],
                vec!["..*", ".3.", "..."],
                vec!["...", "*3.", "..."],
                vec!["...", ".3*", "..."],
                vec!["...", ".3.", "*.."],
                vec!["...", ".3.", ".*."],
                vec!["...", ".3.", "..*"],
            ];
            for i in input {
                assert!(check_surrounds(i, Vec2::new(1, 1)));
            }
        }

        #[test]
        fn above() {
            let input = vec!["...*..", "...4.."];
            assert!(check_surrounds(input, Vec2 { x: 3, y: 1 }));
        }

        #[test]
        fn below() {
            let input = vec!["...4..", "...*.."];
            assert!(check_surrounds(input, Vec2 { x: 3, y: 0 }));
        }

        #[test]
        fn side() {
            let input = vec!["...4*.", "......"];
            assert!(check_surrounds(input, Vec2 { x: 3, y: 0 }));
        }

        #[test]
        fn diagnol() {
            let input = vec!["...4..", "....*."];
            assert!(check_surrounds(input, Vec2 { x: 3, y: 0 }));
        }

        #[test]
        fn none() {
            let input = vec!["...4..", "......"];
            assert!(!check_surrounds(input, Vec2 { x: 3, y: 0 }));
        }
    }
}
