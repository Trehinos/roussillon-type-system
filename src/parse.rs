
pub type Parsed<'a, T> = (Option<T>, &'a [u8]);

pub fn parse_slice(input: &[u8], len: usize) -> Parsed<&[u8]> {
    if input.len() < len {
        (None, input)
    } else {
        let (raw, rest) = input.split_at(len);
        (Some(raw), rest)
    }
}
