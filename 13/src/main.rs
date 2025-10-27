use std::fs;
mod packet;
fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut packets: Vec<Vec<packet::PacketElem>> = Vec::new();
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut parsed: Vec<packet::ParseState> = Vec::new();
        let mut state = packet::ParseState::Begin;
        for char in line.chars() {
            let next_state = state.next_state(char);
            match (state.clone(), next_state.clone()) {
                (_, packet::ParseState::Digit(_)) => {
                    state = next_state;
                    continue;
                }
                (packet::ParseState::Digit(_), _) => {
                    parsed.push(state);
                    parsed.push(next_state.clone());
                    state = next_state;
                }
                _ => {
                    parsed.push(next_state.clone());
                    state = next_state;
                }
            }
        }
        let (elems, _) = packet::make_packet_list(&parsed);
        packets.push(elems);
    }
    let mut total = 0;
    for (i, chunk) in packets.chunks(2).enumerate() {
        let result = packet::compare_packet_lists(&chunk[0], &chunk[1]).unwrap();
        if result {
            total += i + 1
        }
    }
    println!("Part 1: {}", total);

    let d1 = vec![packet::PacketElem::Value(2)];
    let d2 = vec![packet::PacketElem::Value(6)]; // We don't actually have to fully sort
    let indices = packets.iter().fold((1, 2), |(i1, i2), elem| {
        let mut in1 = i1;
        let mut in2 = i2;
        if packet::compare_packet_lists(elem, &d1).unwrap() {
            in1 += 1;
        }
        if packet::compare_packet_lists(elem, &d2).unwrap() {
            in2 += 1;
        }
        (in1, in2)
    });
    println!("Part 2: {}", indices.0 * indices.1);
}
