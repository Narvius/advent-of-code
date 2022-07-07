use std::collections::HashSet;

/// Count the number of IPv7s that support TLS.
pub fn one(input: &str) -> crate::Result<String> {
    fn supports_tls(s: &&str) -> bool {
        let mut in_hypernet = false;
        let mut has_abba = false;
        let mut has_hyper_abba = false;

        for w in s.as_bytes().windows(4) {
            let is_abba = w[0] != w[1] && [w[0], w[1]] == [w[3], w[2]];
            match (w[0], is_abba, in_hypernet) {
                (b'[', _, _) => in_hypernet = true,
                (b']', _, _) => in_hypernet = false,
                (_, true, true) => has_hyper_abba = true,
                (_, true, false) => has_abba = true,
                _ => {}
            }
        }

        has_abba && !has_hyper_abba
    }

    Ok(input.lines().filter(supports_tls).count().to_string())
}

/// Count the number of IPv7s that support SSL.
pub fn two(input: &str) -> crate::Result<String> {
    fn supports_ssl(s: &&str) -> bool {
        let mut in_hypernet = false;
        let mut aba = HashSet::new();
        let mut bab = HashSet::new();

        for w in s.as_bytes().windows(3) {
            let is_aba = w[0] != w[1] && w[0] == w[2];
            match (w[0], is_aba, in_hypernet) {
                (b'[', _, _) => in_hypernet = true,
                (b']', _, _) => in_hypernet = false,
                (_, true, true) => bab.extend([(w[1], w[0])]),
                (_, true, false) => aba.extend([(w[0], w[1])]),
                _ => {}
            }
        }

        aba.intersection(&bab).any(|_| true)
    }

    Ok(input.lines().filter(supports_ssl).count().to_string())
}
