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
    for i in 1..l {
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