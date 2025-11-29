
pub fn create<'a>(symbols_per_card: u8) -> Result<Vec<Vec<u8>>, &'static str> {
    if symbols_per_card < 3 {
        return Err("Dobble can be constructor only when we have more that 3 symbols per card");
    }

    let max_card_count = (symbols_per_card - 1) * (symbols_per_card - 1) + (symbols_per_card - 1) + 1;

    println!("With {} symbols per card, we can create maximum {} cards", symbols_per_card, max_card_count);

    // Initialize empty array
    let mut used_symbols = Vec::new();
    for n in 1..max_card_count {
        let mut vec2 = Vec::new();
        for i in 1..symbols_per_card-1 {
            vec2.push(0);
        }
        used_symbols.push(vec2);
    }

    for symbol in 1..max_card_count {
        
    }


    Ok(used_symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_minimum_fail() {
        let result = create(2);
        assert!(result.is_err());

    }

    #[test]
    fn check_minimum_ok() {
        let result = create(3);
        assert!(result.is_ok());
    }

    #[test]
    fn check_minimum_ok() {
        let result = create(3);
        assert!(result.is_ok());
    }
}
