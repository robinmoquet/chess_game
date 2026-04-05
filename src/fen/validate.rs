use regex::Regex;

pub fn is_valid(fen: &String) -> bool {
    let parts: Vec<&str> = fen.split(" ").collect();
    if parts.len() != 6 {
        return false;
    }

    let mut parts = parts.iter();
    if !Regex::new(r"^([rnbqkpRNBQKP1-8]{1,8}\/){7}([rnbqkpRNBQKP1-8]{1,8})$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }

    if !Regex::new(r"^[wb]{1}$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }
    if !Regex::new(r"^[KQkq-]{1,4}$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }
    if !Regex::new(r"^([a-h][1-8])|(-)$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }
    if !Regex::new(r"^[0-9]{1,3}$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }
    if !Regex::new(r"^[0-9]{1,3}$")
        .unwrap()
        .is_match(parts.next().unwrap())
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::fen::validate::is_valid;

    #[test]
    fn empty() {
        assert_eq!(false, is_valid(&String::new()));
    }

    #[test]
    fn bad_format() {
        assert_eq!(false, is_valid(&String::from("foobar")));
        assert_eq!(false, is_valid(&String::from("foo bar foo bar foo bar")));
        assert_eq!(
            false,
            is_valid(&String::from(
                "0B6/2n5/p1N1P2R/P1K3N1/4Pk0/1Q2p2p/6nP/1B4R1 w - - 0 1"
            )),
            "0 is not valid for FEN pieces position"
        );
        assert_eq!(
            false,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KRkr - 0 1"
            )),
            "Rook is not valid for castling notation"
        );
        assert_eq!(
            false,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR 0 KQkq - 0 1"
            )),
            "Active color must be w or b"
        );
        assert_eq!(
            false,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e 0 1"
            )),
            "en passant, must be empty - or valid position"
        );
        assert_eq!(
            false,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - t 1"
            )),
            "halfmove clock must be a valid number"
        );
        assert_eq!(
            false,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 a"
            )),
            "fullmove number must be valid number"
        );
    }

    #[test]
    fn valid() {
        assert_eq!(
            true,
            is_valid(&String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            ))
        );
        assert_eq!(
            true,
            is_valid(&String::from(
                "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
            ))
        );
        assert_eq!(
            true,
            is_valid(&String::from(
                "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
            ))
        );
        assert_eq!(
            true,
            is_valid(&String::from(
                "1B6/2n5/p1N1P2R/P1K3N1/4Pk2/1Q2p2p/6nP/1B4R1 w - - 0 1"
            ))
        );
    }
}
