use std::io;

type Card = u8;
type HandType = u8;

fn card(c: char) -> Card {
    match c {
        // Joker => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Bad card"),
    }
}

fn hand_type(hand: &[Card; 5]) -> HandType {
    let mut counts: [usize; 14] = [0; 14];
    for card in hand { counts[*card as usize] += 1 }
    let jokers = counts[0];
    counts[0] = 0;
    counts.sort();
    // It's always best to pretend that the jokers are whatever card we have the
    // most of.
    counts[13] += jokers;
    match counts {
        [.., 1] => 0,     // High card
        [.., 1, 2] => 1,  // One pair
        [.., 2, 2] => 2,  // Two pairs
        [.., 1, 3] => 3,  // Three of a kind
        [.., 2, 3] => 4,  // Full house
        [.., 4] => 5,     // Four of a kind
        [.., 5] => 6,     // Five of a kind
        _ => panic!("Impossible hand"),
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

fn hand(s: &str) -> Hand {
    let cards: [Card; 5] = s
        .chars()
        .map(|c| card(c))
        .collect::<Vec<Card>>()  // This allocation makes me sad :(
        .try_into()
        .unwrap();
    return Hand{hand_type: hand_type(&cards), cards: cards};
}

fn record(s: &str) -> (Hand, u32) {
    let (h, b) = s.split_once(" ").unwrap();
    return (hand(h), b.parse().unwrap());
}

fn winnings(records: &[(Hand, u32)]) -> u32 {
    let mut total = 0;
    for (rank, (_, bet)) in (1..).zip(records) {
        total += rank * bet;
    }
    return total;
}

fn main() {
    let mut records: Vec<(Hand, u32)> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| record(&s))
        .collect();
    records.sort();
    let part1 = winnings(&records);
    for (hand, _) in &mut records {
        // Rewrite Jacks as Jokers.
        for c in &mut hand.cards {
            if *c == card('J') { *c = 0 }
        }
        // Re-determine hand types.
        hand.hand_type = hand_type(&hand.cards);
    }
    records.sort();
    let part2 = winnings(&records);
    print!("{}\n{}\n", part1, part2);
}
