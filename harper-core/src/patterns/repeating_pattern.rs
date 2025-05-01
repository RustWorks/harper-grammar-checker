use super::Pattern;
use crate::Token;

/// A pattern that will match one or more repetitions of the same pattern.
///
/// Somewhat reminiscent of the `.*` operator in Regex.
pub struct RepeatingPattern {
    inner: Box<dyn Pattern>,
    required_repetitions: usize,
}

impl RepeatingPattern {
    pub fn new(pattern: Box<dyn Pattern>, required_repetitions: usize) -> Self {
        Self {
            inner: pattern,
            required_repetitions,
        }
    }
}

impl Pattern for RepeatingPattern {
    fn matches(&self, tokens: &[Token], source: &[char]) -> Option<usize> {
        let mut tok_cursor = 0;
        let mut repetition = 0;

        loop {
            let match_len = self.inner.matches(&tokens[tok_cursor..], source);

            if let Some(match_len) = match_len {
                if match_len == 0 {
                    // If match_len == 0, we won't move forward ever again.
                    // This means that we can get infinitely many repetitions,
                    // so repetition >= self.required_repetitions is guaranteed.
                    return Some(tok_cursor);
                }

                tok_cursor += match_len;
                repetition += 1;
            } else if repetition >= self.required_repetitions {
                return Some(tok_cursor);
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::RepeatingPattern;
    use crate::Document;
    use crate::patterns::{AnyPattern, Pattern};

    #[test]
    fn matches_anything() {
        let doc = Document::new_plain_english_curated(
            "This matcher will match the entirety of any document!",
        );
        let pat = RepeatingPattern::new(Box::new(AnyPattern), 0);

        assert_eq!(
            pat.matches(doc.get_tokens(), doc.get_source()),
            Some(doc.get_tokens().len())
        )
    }

    #[test]
    fn does_not_match_short() {
        let doc = Document::new_plain_english_curated("No match");
        let pat = RepeatingPattern::new(Box::new(AnyPattern), 4);

        assert_eq!(pat.matches(doc.get_tokens(), doc.get_source()), None)
    }
}
