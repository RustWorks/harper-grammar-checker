use super::Pattern;

pub struct WhitespacePattern;

impl Pattern for WhitespacePattern {
    fn matches(&self, tokens: &[crate::Token], _source: &[char]) -> Option<usize> {
        let count = tokens
            .iter()
            .position(|t| !t.kind.is_whitespace())
            .unwrap_or(tokens.len());

        if count == 0 { None } else { Some(count) }
    }
}
