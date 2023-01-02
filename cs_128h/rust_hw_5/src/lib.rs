/// TODO: Implement the mark_cards function
/// Marks every 3rd card by adding M to the front of the card's 
/// string representation (irrespective of if it already has a M or not)
/// Make sure to derefence cards before changing it
pub fn mark_cards(cards: &mut Vec<String>) {
    let mut i = 0;
    for card in cards {
        if i == 2 {
            *card = ["M", card.as_str()].join("");
            i = 0;
        } else {
            i += 1;
        }
    }
}

/// TODO: Implement the perfect_bridge function
/// Takes a Vector of strings representing a deck of cards (will always have an even number) 
/// Divides the deck into two parts at the middle, then shuffles the cards 
/// Such that the first card of the first half resides in the 0th position 
/// and the first card of the second hald resides in the 1th position
/// And the second card of the first half resides in the 3rd position etc.
/// After shuffling return the deck
pub fn perfect_bridge(cards: &Vec<String>) -> Vec<String> {
    let mut bridge: Vec<String> = Vec::with_capacity(cards.len());
    let (mut i, mut j) = (0 as usize, 0 as usize);
    while i + j < cards.len() {
        if i == j {
            bridge.push(cards[i].clone());
            i += 1;
        } else {
            bridge.push(cards[cards.len() / 2 + j].clone());
            j += 1;
        }
    }
    bridge
}

/// TODO: Implement the runner function
/// Takes a vector of strings representing a deck of cards
/// Shuffle the cards, then mark the cards, and finally shuffle them again
pub fn runner(cards: &mut Vec<String>) {
    *cards = perfect_bridge(&cards);
    mark_cards(cards);
    *cards = perfect_bridge(&cards);
}

pub fn make_string_vec(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|c| c.to_string()).collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mark_cards_none_marked() {
        let mut cards = make_string_vec( vec!["A", "B"] );
        mark_cards(&mut cards);
        assert_eq!(cards, make_string_vec( vec!["A", "B"] ));
    }
    
    #[test]
    fn mark_cards_one_marked() {
        let mut cards = make_string_vec( vec!["A", "B", "C"] );
        mark_cards(&mut cards);
        assert_eq!(cards, make_string_vec( vec!["A", "B", "MC"] ));
    }
    
    #[test]
    fn mark_cards_two_marked() {
        let mut cards = make_string_vec( vec!["A", "B", "C", "A", "C", "B"] );
        mark_cards(&mut cards);
        assert_eq!(cards, make_string_vec( vec!["A", "B", "MC", "A", "C", "MB"] ));
    }

    #[test]
    fn mark_cards_two_marked_extra() {
        let mut cards = make_string_vec( vec!["A", "B", "C", "A", "C", "B", "X", "Y"] );
        mark_cards(&mut cards);
        assert_eq!(cards, make_string_vec( vec!["A", "B", "MC", "A", "C", "MB", "X", "Y"] ));
    }

    #[test]
    fn perfect_bridge_two_cards() {
        let cards = make_string_vec( vec!["A", "C"] );
        let bridged = perfect_bridge(&cards);
        assert_eq!(bridged, make_string_vec( vec!["A", "C"] ));
    }

    #[test]
    fn perfect_bridge_four_cards() {
        let cards = make_string_vec( vec!["A", "B", "C", "A"] );
        let bridged = perfect_bridge(&cards);
        assert_eq!(bridged, make_string_vec( vec!["A", "C", "B", "A"] ));
    }
    
    #[test]
    fn perfect_bridge_six_cards() {
        let cards = make_string_vec( vec!["A", "B", "C", "A", "X", "Y"] );
        let bridged = perfect_bridge(&cards);
        assert_eq!(bridged, make_string_vec( vec!["A", "A", "B", "X", "C", "Y"] ));
    }
}