use std::collections::HashMap;

/// Find the number of strings matching rule 0.
pub fn one(input: &str) -> crate::Result<usize> {
    let (a, b, rules, lines) = parse(input).ok_or("parse failed")?;
    let regex = compile_to_regex(a, b, rules, false)?;
    Ok(lines.filter(|line| regex.is_match(line)).count())
}

/// Find the number of strings matching rule 0, if rules #8 and #11 can loop.
pub fn two(input: &str) -> crate::Result<usize> {
    let (a, b, rules, lines) = parse(input).ok_or("parse failed")?;
    let regex = compile_to_regex(a, b, rules, true)?;
    Ok(lines.filter(|line| regex.is_match(line)).count())
}

type Cache = Vec<Option<String>>;
type Rules = HashMap<usize, Rule>;
type Rule = (Vec<usize>, Option<Vec<usize>>);

/// Returns the [`Regex`](regex::Regex) corresponding to a fully-expanded rule 0 from the input.
/// `a` and `b` are the numbers of the raw "a" and "b" rules.
/// If `loops` is set, rules #8 and #11 are expanded to support looping.
fn compile_to_regex(a: usize, b: usize, rules: Rules, loops: bool) -> crate::Result<regex::Regex> {
    let mut cache = vec![None; rules.len() + 2];
    cache[a] = Some("a".to_string());
    cache[b] = Some("b".to_string());

    if loops {
        cache[8] = Some(unrolled_loop(&mut cache, &rules, 42, None));
        cache[11] = Some(unrolled_loop(&mut cache, &rules, 42, Some(31)));
    }

    Ok(regex::Regex::new(&format!(
        "^{}$",
        rule_regex(&mut cache, &rules, 0).replace(' ', "")
    ))?)
}

/// Gets the regex string for a given rule, calculating and storing it in the cache first if
/// it doesn't exist in there yet.
fn rule_regex<'a>(cache: &'a mut Cache, rules: &Rules, i: usize) -> &'a str {
    if cache[i].is_some() {
        return cache[i].as_ref().unwrap().as_ref();
    }

    let (main, alt) = rules.get(&i).unwrap();
    let mut val = String::from("(?:");

    for &i in main {
        val.push_str(rule_regex(cache, rules, i));
    }
    if let Some(alt) = alt {
        val.push('|');
        for &i in alt {
            val.push_str(rule_regex(cache, rules, i));
        }
    }
    val.push(')');

    cache[i] = Some(val);
    cache[i].as_ref().unwrap().as_ref()
}

/// Builds a regex string that that contains branches for 1 to 5 repetitions of the `pre` and
/// `post` regexes.
fn unrolled_loop(cache: &mut Cache, rules: &Rules, pre: usize, post: Option<usize>) -> String {
    const ITERATIONS: i32 = 5;
    if let Some(post) = post {
        let mut result = String::new();
        for _ in 1..=ITERATIONS {
            result.push_str("(?:");
            result.push_str(rule_regex(cache, rules, pre));
        }
        for _ in 1..=ITERATIONS {
            result.push_str(rule_regex(cache, rules, post));
            result.push_str(")??");
        }
        result.truncate(result.len() - 2);
        result
    } else {
        format!("{}{{1,{ITERATIONS}}}", rule_regex(cache, rules, pre))
    }
}

/// Parses the puzzle input, resulting in the numbers of the "a" and "b" rules, a collection
/// of all the other rules, and an iterator over the lines to match against.
fn parse(input: &str) -> Option<(usize, usize, Rules, impl Iterator<Item = &str> + '_)> {
    fn list(s: &str) -> Option<Vec<usize>> {
        s.split_ascii_whitespace().map(|v| v.parse().ok()).collect()
    }

    let mut lines = input.lines();

    let mut rule_a = None;
    let mut rule_b = None;
    let mut rules = HashMap::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (id, rule) = line.split_once(": ")?;

        match rule {
            "\"a\"" => rule_a = Some(id.parse::<usize>().ok()?),
            "\"b\"" => rule_b = Some(id.parse::<usize>().ok()?),
            _ => {
                rules.entry(id.parse::<usize>().ok()?).or_insert(
                    if let Some((main, alt)) = rule.split_once(" | ") {
                        (list(main)?, list(alt))
                    } else {
                        (list(rule)?, None)
                    },
                );
            }
        }
    }

    Some((rule_a?, rule_b?, rules, lines))
}
