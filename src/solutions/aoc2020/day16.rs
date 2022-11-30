use std::ops::RangeInclusive;

/// Find the sum of all invalid numbers across all nearby tickets (ones that don't match any
/// field).
pub fn one(input: &str) -> crate::Result<i32> {
    let (fields, _, tickets) = parse(input)?;

    Ok(tickets
        .into_iter()
        .flat_map(|vs| vs.into_iter())
        .filter(|&v| fields.iter().all(|field| !field.matches(v)))
        .sum())
}

/// Multiply together the six departure fields on your own ticket. For that, we must identify
/// which fields correspond to which position on the ticket.
pub fn two(input: &str) -> crate::Result<i64> {
    fn valid_ticket(ticket: &Ticket, fields: &[Field]) -> bool {
        ticket.iter().all(|&v| fields.iter().any(|f| f.matches(v)))
    }

    let (fields, own, mut tickets) = parse(input)?;

    // Throw out all invalid tickets.
    tickets.retain(|ticket| valid_ticket(ticket, &fields));

    // For each position, find all possible fields it might be.
    let mut candidates: Vec<Vec<usize>> = (0..fields.len())
        .map(|i| {
            fields
                .iter()
                .enumerate()
                .filter(|(_, f)| tickets.iter().all(|t| f.matches(t[i])))
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    // Find candidate lists that only have 1 entry, those are now fixed. Then remove that field
    // from all other candidate lists. Repeat until solved.
    let mut ordered = vec![0; fields.len()];
    while let Some(n) = candidates.iter().position(|v| v.len() == 1) {
        ordered[n] = candidates[n][0];
        for candidate_list in &mut candidates {
            candidate_list.retain(|&v| v != ordered[n]);
        }
    }

    // Multiply together all departure fields from your own ticket.
    Ok(ordered
        .into_iter()
        .enumerate()
        .filter(|&(_, f)| fields[f].name.starts_with("departure"))
        .map(|(i, _)| own[i] as i64)
        .product())
}

type Ticket = Vec<i32>;

/// A ticket field, made of a name and two number ranges.
struct Field<'a> {
    name: &'a str,
    range_one: RangeInclusive<i32>,
    range_two: RangeInclusive<i32>,
}

impl Field<'_> {
    /// Checks whether a number matches this ticket field (ie. is contained in either range).
    fn matches(&self, number: i32) -> bool {
        self.range_one.contains(&number) || self.range_two.contains(&number)
    }
}

/// Parses all three sections of the puzzle input (ticket field list, your own ticket, nearby
/// tickets).
fn parse(input: &str) -> crate::Result<(Vec<Field>, Ticket, Vec<Ticket>)> {
    let mut lines = input.lines();

    fn range(s: &str) -> crate::Result<RangeInclusive<i32>> {
        let (min, max) = s.split_once('-').ok_or("parse failed")?;
        Ok(min.parse()?..=max.parse()?)
    }

    fn ticket(s: &str) -> crate::Result<Ticket> {
        Ok(s.split(',').map(|v| v.parse()).collect::<Result<_, _>>()?)
    }

    let mut fields = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (name, ranges) = line.split_once(": ").ok_or("parse failed")?;
        let (a, b) = ranges.split_once(" or ").ok_or("parse failed")?;
        fields.push(Field {
            name,
            range_one: range(a)?,
            range_two: range(b)?,
        });
    }

    lines.next();
    let own = ticket(lines.next().ok_or("unexpected end of input")?)?;
    let other = lines.skip(2).map(ticket).collect::<Result<Vec<_>, _>>()?;

    Ok((fields, own, other))
}
