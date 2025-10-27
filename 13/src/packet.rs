#[derive(Debug)]
pub enum PacketElem {
    Value(usize),
    List(Vec<PacketElem>),
}

pub fn make_packet_list(data: &[ParseState]) -> (Vec<PacketElem>, usize) {
    match data[0] {
        ParseState::ListStart => (),
        _ => panic!(),
    }
    let mut list: Vec<PacketElem> = Vec::new();
    let mut values: Vec<usize> = Vec::new();
    let mut to_skip: usize = 0;

    for (i, state) in data.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if to_skip > 0 {
            to_skip -= 1;
            continue;
        }
        match state {
            ParseState::Digit(d) => values.push(*d),
            ParseState::ListEnd => {
                if values.len() > 0 {
                    let value = get_vec_value(&values);
                    list.push(PacketElem::Value(value))
                }
                return (list, i + 1);
            }
            ParseState::ListStart => {
                let (sublist, n) = make_packet_list(&data[i..]);
                list.push(PacketElem::List(sublist));
                to_skip = n;
            }
            ParseState::Comma => {
                if values.len() > 0 {
                    let value = get_vec_value(&values);
                    list.push(PacketElem::Value(value))
                }
                values = Vec::new();
            }
            _ => panic!(),
        }
    }
    return (list, data.len());
}

pub fn compare_packet_lists(
    left_list: &Vec<PacketElem>,
    right_list: &Vec<PacketElem>,
) -> Option<bool> {
    for (i, left) in left_list.iter().enumerate() {
        let right_val_ = right_list.get(i);
        if right_val_.is_none() {
            return Some(false);
        }
        let right = right_val_.unwrap();
        match (left, right) {
            (PacketElem::Value(v1), PacketElem::Value(v2)) => {
                if v1 == v2 {
                    continue;
                } else if v1 < v2 {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
            (PacketElem::Value(vl), PacketElem::List(lr)) => {
                let new_left = vec![PacketElem::Value(*vl)];
                if let Some(res) = compare_packet_lists(&new_left, lr) {
                    return Some(res);
                }
            }
            (PacketElem::List(ll), PacketElem::Value(vr)) => {
                let new_right = vec![PacketElem::Value(*vr)];
                if let Some(res) = compare_packet_lists(ll, &new_right) {
                    return Some(res);
                }
            }
            (PacketElem::List(ll), PacketElem::List(lr)) => {
                let res = compare_packet_lists(ll, lr);
                if res.is_some() {
                    return res;
                }
            }
        }
    }
    if left_list.len() < right_list.len() {
        return Some(true);
    }
    return None;
}

pub fn compare_packets(left: &PacketElem, right: &PacketElem) -> bool {
    match (left, right) {
        (PacketElem::Value(v1), PacketElem::Value(v2)) => v1 <= v2,
        (PacketElem::Value(v1), PacketElem::List(_)) => {
            let new_left = PacketElem::List(vec![PacketElem::Value(*v1)]);
            compare_packets(&new_left, right)
        }
        (PacketElem::List(_), PacketElem::Value(v2)) => {
            let new_right = PacketElem::List(vec![PacketElem::Value(*v2)]);
            compare_packets(left, &new_right)
        }
        (PacketElem::List(l1), PacketElem::List(l2)) => {
            if l1.len() > l2.len() {
                return false;
            }
            for (i, item) in l1.iter().enumerate() {
                if !compare_packets(item, &l2[i]) {
                    return false;
                }
            }
            return true;
        }
    }
}

fn get_vec_value(vals: &[usize]) -> usize {
    vals.iter().enumerate().fold(0, |acc, (i, val)| {
        acc + val * (10 as u32).pow(i as u32) as usize
    })
}

#[derive(Debug, Clone)]
pub enum ParseState {
    Begin,
    ListStart,
    Digit(usize),
    Comma,
    ListEnd,
    Invalid,
}

impl ParseState {
    pub fn next_state(&self, next_char: char) -> ParseState {
        match self {
            ParseState::Begin => self.transition_begin(next_char),
            ParseState::ListStart => self.transition_list_start(next_char),
            ParseState::Digit(d) => self.transition_digit(next_char, *d),
            ParseState::Comma => self.transition_comma(next_char),
            ParseState::ListEnd => self.transition_list_end(next_char),
            ParseState::Invalid => panic!(),
        }
    }
    fn transition_begin(&self, next_char: char) -> ParseState {
        if next_char == '[' {
            ParseState::ListStart
        } else {
            ParseState::Invalid
        }
    }

    fn transition_list_start(&self, next_char: char) -> ParseState {
        if next_char == '[' {
            ParseState::ListStart
        } else if next_char == ']' {
            ParseState::ListEnd
        } else if let Some(d) = next_char.to_digit(10) {
            ParseState::Digit(d as usize)
        } else {
            ParseState::Invalid
        }
    }
    fn transition_digit(&self, next_char: char, val: usize) -> ParseState {
        if let Some(d) = next_char.to_digit(10) {
            return ParseState::Digit(10 * val + d as usize);
        }
        if next_char == ',' {
            ParseState::Comma
        } else if next_char == ']' {
            ParseState::ListEnd
        } else {
            ParseState::Invalid
        }
    }
    fn transition_comma(&self, next_char: char) -> ParseState {
        if next_char == '[' {
            ParseState::ListStart
        } else if let Some(d) = next_char.to_digit(10) {
            ParseState::Digit(d as usize)
        } else {
            ParseState::Invalid
        }
    }
    fn transition_list_end(&self, next_char: char) -> ParseState {
        if next_char == ',' {
            ParseState::Comma
        } else if next_char == ']' {
            ParseState::ListEnd
        } else {
            ParseState::Invalid
        }
    }
}
