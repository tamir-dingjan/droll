use clap::Parser;

/// Roll the specified dice and report the total, individual roles, and percentage chance of the result.
#[derive(Parser)]
struct Cli {
    /// Dice specifications (e.g., 1d6, 2d4+3)
    #[arg(required = true, help = "Dice specifications (e.g., 1d6, 2d4+3)")]
    dice: Vec<String>,
}

#[derive(Debug)]
struct Dice {
    sides: u8,
    count: u8,
    modifier: i32,
}

impl Dice {
    fn parse(spec: &str) -> Result<Self, String> {
        // Trim whitespace
        let spec = spec.trim().to_lowercase();

        // Split the count and side values by "d"
        let parts: Vec<&str> = spec.split('d').collect();

        if parts.len() != 2 {
            return Err(format!(
                "Invalid dice specification '{}': must be in format 'NdS' (e.g., '2d6')",
                spec
            ));
        }

        let count = parts[0].parse::<u8>().map_err(|_| {
            format!(
                "Invalid count in '{}': '{}' is not a valid number",
                spec, parts[0]
            )
        })?;

        if count == 0 {
            return Err(format!("Invalid count in '{}': cannot use 0 dice", spec));
        }

        // Parse sides and modifier from the second part
        let (sides, modifier) = if parts[1].contains('+') {
            let mut split = parts[1].split('+');
            let sides_str = split.next().unwrap();
            let modifier_str = split.next().unwrap_or("0");

            let sides = sides_str.parse::<u8>().map_err(|_| {
                format!(
                    "Invalid sides in '{}': '{}' is not a valid number",
                    spec, sides_str
                )
            })?;

            let modifier = modifier_str.parse::<i32>().map_err(|_| {
                format!(
                    "Invalid modifier in '{}': '{}' is not a valid number",
                    spec, modifier_str
                )
            })?;

            (sides, modifier)
        } else if parts[1].contains('-') {
            let mut split = parts[1].split('-');
            let sides_str = split.next().unwrap();
            let modifier_str = split.next().unwrap_or("0");

            let sides = sides_str.parse::<u8>().map_err(|_| {
                format!(
                    "Invalid sides in '{}': '{}' is not a valid number",
                    spec, sides_str
                )
            })?;

            let modifier = modifier_str.parse::<i32>().map_err(|_| {
                format!(
                    "Invalid modifier in '{}': '{}' is not a valid number",
                    spec, modifier_str
                )
            })?;

            (sides, -modifier) // Make the modifier negative
        } else {
            let sides = parts[1].parse::<u8>().map_err(|_| {
                format!(
                    "Invalid sides in '{}': '{}' is not a valid number",
                    spec, parts[1]
                )
            })?;
            (sides, 0)
        };

        if sides == 0 {
            return Err(format!("Invalid sides in '{}': cannot use 0 sides", spec));
        }

        Ok(Dice {
            sides,
            count,
            modifier,
        })
    }
}

fn main() {
    let args = Cli::parse();
    let mut dice_vec = Vec::new();

    for spec in &args.dice {
        match Dice::parse(spec) {
            Ok(dice) => dice_vec.push(dice),
            Err(err) => {
                eprintln!("Error parsing dice specification '{}': {}", spec, err);
                std::process::exit(1);
            }
        }
    }
    println!("Dice to roll: {:?}", dice_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_dice() {
        let dice = Dice::parse("1d6").unwrap();
        assert_eq!(dice.count, 1);
        assert_eq!(dice.sides, 6);
        assert_eq!(dice.modifier, 0);
    }

    #[test]
    fn test_parse_multiple_dice() {
        let dice = Dice::parse("3d8").unwrap();
        assert_eq!(dice.count, 3);
        assert_eq!(dice.sides, 8);
        assert_eq!(dice.modifier, 0);
    }

    #[test]
    fn test_parse_dice_with_positive_modifier() {
        let dice = Dice::parse("2d10+5").unwrap();
        assert_eq!(dice.count, 2);
        assert_eq!(dice.sides, 10);
        assert_eq!(dice.modifier, 5);
    }

    #[test]
    fn test_parse_dice_with_negative_modifier() {
        let dice = Dice::parse("1d20-3").unwrap();
        assert_eq!(dice.count, 1);
        assert_eq!(dice.sides, 20);
        assert_eq!(dice.modifier, -3);
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let dice = Dice::parse("  2D6+1  ").unwrap();
        assert_eq!(dice.count, 2);
        assert_eq!(dice.sides, 6);
        assert_eq!(dice.modifier, 1);
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(Dice::parse("invalid").is_err());
        assert!(Dice::parse("2x6").is_err());
        assert!(Dice::parse("d6").is_err());
        assert!(Dice::parse("2d").is_err());
    }

    #[test]
    fn test_parse_invalid_numbers() {
        assert!(Dice::parse("abc d6").is_err());
        assert!(Dice::parse("2d abc").is_err());
        assert!(Dice::parse("2d6+ abc").is_err());
    }

    #[test]
    fn test_parse_zero_values() {
        assert!(Dice::parse("0d6").is_err());
        assert!(Dice::parse("2d0").is_err());
    }
}
