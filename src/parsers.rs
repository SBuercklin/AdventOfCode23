use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    multi::many0,
    sequence::terminated,
    IResult,
};

pub fn comma_separated(s: &str) -> IResult<&str, Vec<&str>> {
    let (rem, parsed) = many0(alt((terminated(is_not(","), tag(",")), is_not(","))))(s)?;

    return Ok((rem, parsed));
}

pub fn space_separated(s: &str) -> IResult<&str, Vec<&str>> {
    let (rem, parsed) = many0(alt((terminated(is_not(" "), tag(" ")), is_not(" "))))(s)?;

    return Ok((rem, parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv() {
        let input = ".?3#33.3,92,ASB".to_string();

        let (_, result) = comma_separated(&input).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ".?3#33.3".to_string());
        assert_eq!(result[1], "92".to_string());
        assert_eq!(result[2], "ASB".to_string());
    }
    #[test]
    fn test_ssv() {
        let input = ".?3#33.3 92 ASB".to_string();

        let (_, result) = space_separated(&input).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ".?3#33.3".to_string());
        assert_eq!(result[1], "92".to_string());
        assert_eq!(result[2], "ASB".to_string());
    }
}
