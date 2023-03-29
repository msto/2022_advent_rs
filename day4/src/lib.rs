use std::ops::Range;

fn parse_range(s: &str) -> Range<i32> {
    let r: Vec<i32> = s.split("-").filter_map(|x| x.parse::<i32>().ok()).collect();

    Range {
        start: r[0],
        end: r[1] + 1,
    }
}

pub fn parse_ranges(line: &str) -> Vec<Range<i32>> {
    line.split(",").map(|x| parse_range(x)).collect()
}

fn is_contained(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    r1.contains(&r2.start) && r1.contains(&(&r2.end - 1))
}

pub fn either_contains(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    is_contained(r1, r2) || is_contained(r2, r1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let start = 2;
        let end = 6;
        let r = parse_range(&format!("{}-{}", start, end));

        assert_eq!(r.start, start);
        assert_eq!(r.end, end + 1);
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges("2-6,3-4");

        assert_eq!(ranges[0].start, 2);
        assert_eq!(ranges[0].end, 7);
        assert_eq!(ranges[1].start, 3);
        assert_eq!(ranges[1].end, 5);
    }

    #[test]
    fn test_is_contained() {
        let r1 = Range { start: 2, end: 7 };
        let r2 = Range { start: 6, end: 7 };
        assert!(is_contained(&r1, &r2));
    }
}
