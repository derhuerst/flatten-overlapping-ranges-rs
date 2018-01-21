use std::collections::BTreeSet;

// helper fn
fn sorted_insert(vec: &mut Vec<usize>, val: usize) -> usize {
    let i = vec.binary_search(&val).unwrap_or_else(|i| i);
    vec.insert(i, val);
    return i;
}

enum MarkerType {
    Start,
    Stop,
}

pub fn flatten(ranges: &Vec<(&char, usize, usize)>) -> Vec<(usize, Vec<char>)> {
    let mut indexes: Vec<usize> = Vec::new();
    let mut ids: Vec<&char> = Vec::new();
    let mut types: Vec<MarkerType> = Vec::new();

    for range in ranges {
        let start_i = sorted_insert(&mut indexes, range.1);
        ids.insert(start_i, range.0);
        types.insert(start_i, MarkerType::Start);

        let end_i = sorted_insert(&mut indexes, range.1 + range.2);
        ids.insert(end_i, range.0);
        types.insert(end_i, MarkerType::Stop);
    }

    let mut sections: Vec<(usize, Vec<char>)> = Vec::new();
    let mut state = BTreeSet::new();
    // todo: return early if ids has 0 entries
    state.insert(ids[0]);

    let l = indexes.len() - 1;
    for i in 1..(l + 1) {
        sections.push((
            indexes[i] - indexes[i - 1],
            state.iter().map(|&id_ref| *id_ref).collect()
        ));
        match types[i] {
            MarkerType::Start => {
                state.insert(ids[i]);
            }
            MarkerType::Stop => {
                state.remove(ids[i]);
            }
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::flatten;

    #[test]
    fn simple_example() {
        let a = 'a';
        let b = 'b';
        let c = 'c';
        let d = 'd';

        let simple: Vec<(&char, usize, usize)> = vec![
            (&a, 0, 7),
            (&b, 2, 12),
            (&c, 5, 5),
            (&d, 12, 7)
        ];
        let mut res = flatten(&simple);

        assert_eq!(res[0].0, 2);
        assert_eq!(res[0].1.len(), 1);
        assert_eq!(res[0].1[0], a);

        assert_eq!(res[1].0, 3);
        assert_eq!(res[1].1.len(), 2);
        res[1].1.sort();
        assert_eq!(res[1].1[0], a);
        assert_eq!(res[1].1[1], b);

        assert_eq!(res[2].0, 2);
        assert_eq!(res[2].1.len(), 3);
        res[2].1.sort();
        assert_eq!(res[2].1[0], a);
        assert_eq!(res[2].1[1], b);
        assert_eq!(res[2].1[2], c);

        assert_eq!(res[3].0, 3);
        assert_eq!(res[3].1.len(), 2);
        res[3].1.sort();
        assert_eq!(res[3].1[0], b);
        assert_eq!(res[3].1[1], c);

        assert_eq!(res[4].0, 2);
        assert_eq!(res[4].1.len(), 1);
        assert_eq!(res[4].1[0], b);

        assert_eq!(res[5].0, 2);
        assert_eq!(res[5].1.len(), 2);
        res[5].1.sort();
        assert_eq!(res[5].1[0], b);
        assert_eq!(res[5].1[1], d);

        assert_eq!(res[6].0, 5);
        assert_eq!(res[6].1.len(), 1);
        assert_eq!(res[6].1[0], d);
    }
}