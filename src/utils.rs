/// Extracts all YDKE URLs from a string.
/// 
/// # Example
/// 
/// ```
/// use ydke_parser::extract_urls;
/// 
/// let text = "some text ydke://y+iNAcvojQHL6I0B3e45ArAj3gCwI94AsCPeAFP3igNT94oDKpVlASqVZQH+iZwF/omcBf6JnAWyMswFsjLMBbIyzAWom/QBPw0aAj8NGgKQ1sMDkNbDA5DWwwNvvjEEb74xBG++MQSmBe8FeGuUAHhrlAB4a5QA2qwNA9ymHgNohdwFaIXcBWiF3AVxX24BcV9uASJImQAiSJkAIkiZAA==!66qLBcREIQUa3qACyhq/AQq2BgHbgcEDCal6BCyRgQAskYEAPoq9AP60xQP+tMUD/rTFA4iQZwWIkGcF!reIKAq3iCgKt4goCNQeDAjUHgwL83BQBnOG9AJzhvQBL8mcCS/JnAkvyZwILQsYEYHT3BGB09wRgdPcE! more text ydke:///5E1A/+RNQP/kTUDthw3ALYcNwC2HDcAtcqTBHEbUwJxG1MCbGahAWxmoQFsZqEBxvKzAUSt8ANErfADRK3wA5pXhAWaV4QFmleEBVNWbwU/DRoCPw0aAn06UgJ9OlICfTpSAm++MQRvvjEEb74xBFl7YwQ+pHEBPqRxAbsKYQQbeaMFG3mjBRt5owUsvgcELL4HBCy+BwSAqRQEgKkUBA==!9wuWArS2LAOCr/4Auk7ZBcf4TQHERCEFw09BAMAOSQL/JrsCpJorAKkafgBJJZYAMqZvAS2aUQHbI+sD!FAzrARQM6wEUDOsBvzrVAL861QC/OtUAQ77dAEO+3QD73BQBjkjfA45I3wOOSN8DhCV+AIQlfgCEJX4A!";
/// let urls = extract_urls(text);
/// assert_eq!(urls.len(), 2);
/// assert!(urls[0].starts_with("ydke://"));
/// assert!(urls[1].starts_with("ydke://"));
/// ```
pub fn extract_urls(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.starts_with("ydke://"))
        .filter(|url| url.matches('!').count() >= 3)
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_urls() {
        let text = "some text ydke://test1!test2!test3! more text ydke://test4!test5!test6!";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls[0].starts_with("ydke://"));
        assert!(urls[1].starts_with("ydke://"));
    }
}