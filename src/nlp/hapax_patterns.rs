const ARTIFICIAL_TAG_SUFFIXES: [(&str, &str); 10] = [
    ("ing", "SUFF-ING"),
    ("ess", "SUFF-ESS"),
    ("ers", "SUFF-ERS"),
    ("ed", "SUFF-ED"),
    ("es", "SUFF-ES"),
    ("er", "SUFF-ER"),
    ("ly", "SUFF-LY"),
    ("'s", "SUFF-AP-S"),
    ("on", "SUFF-ON"),
    ("le", "SUFF-LE")
];

const ARTIFICIAL_TAG_PREFIXES: [(&str, &str); 9] = [
    ("pro", "PREF-PRO"),
    ("pre", "PREF-PRE"),
    ("dis", "PREF-DIS"),
    ("con", "PREF-CON"),
    ("com", "PREF-COM"),
    ("sta", "PREF-STA"),
    ("re", "PREF-RE"),
    ("in", "PREF-IN"),
    ("un", "PREF-UN")
];

pub fn get_matching_artificial_tag(word: &str) -> Option<&'static str> {
    for (suffix, tag) in ARTIFICIAL_TAG_SUFFIXES {
        if word.ends_with(suffix) {
            return Some(tag.into());
        }
    }

    for (prefix, tag) in ARTIFICIAL_TAG_PREFIXES {
        if word.starts_with(prefix) {
            return Some(tag.into());
        }
    }

    let num_digits = word.chars().filter(char::is_ascii_digit).count();

    if num_digits > word.len() / 2 {
        return Some("IS-A-NUMBER");
    } else if word.chars().any(|c| c == '-') {
        return Some("HAS-MANY-DASHES");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_prefix_matches() {
        assert_eq!(get_matching_artificial_tag("repeal"), Some("PREF-RE"));
        assert_eq!(get_matching_artificial_tag("insidious"), Some("PREF-IN"));
        assert_eq!(get_matching_artificial_tag("understand"), Some("PREF-UN"));
    }

    #[test]
    fn test_long_prefix_matches() {
        assert_eq!(get_matching_artificial_tag("produce"), Some("PREF-PRO"));
        assert_eq!(get_matching_artificial_tag("prefix"), Some("PREF-PRE"));
        assert_eq!(get_matching_artificial_tag("distance"), Some("PREF-DIS"));
        assert_eq!(get_matching_artificial_tag("control"), Some("PREF-CON"));
        assert_eq!(get_matching_artificial_tag("complex"), Some("PREF-COM"));
        assert_eq!(get_matching_artificial_tag("stabilize"), Some("PREF-STA"));
    }

    #[test]
    fn test_short_suffix_matches() {
        assert_eq!(get_matching_artificial_tag("finishes"), Some("SUFF-ES"));
        assert_eq!(get_matching_artificial_tag("finished"), Some("SUFF-ED"));
        assert_eq!(get_matching_artificial_tag("finisher"), Some("SUFF-ER"));
        assert_eq!(get_matching_artificial_tag("happily"), Some("SUFF-LY"));
        assert_eq!(get_matching_artificial_tag("someone's"), Some("SUFF-AP-S"));
        assert_eq!(get_matching_artificial_tag("ton"), Some("SUFF-ON"));
        assert_eq!(get_matching_artificial_tag("ample"), Some("SUFF-LE"));
    }

    #[test]
    fn test_long_suffix_matches() {
        assert_eq!(get_matching_artificial_tag("finishing"), Some("SUFF-ING"));
        assert_eq!(get_matching_artificial_tag("prowess"), Some("SUFF-ESS"));
        assert_eq!(get_matching_artificial_tag("players"), Some("SUFF-ERS"));
    }

    #[test]
    fn test_no_matches() {
        assert_eq!(get_matching_artificial_tag("blahblahblah"), None);
    }
}