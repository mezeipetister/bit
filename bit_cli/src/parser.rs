/**
 * BIT
 * Copyright (C) 2019 Peter Mezei <mezeipetister@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 */

/// Clean line
/// Returns a line vector  
/// - Removes comments
/// - Slice by tabs
pub fn clean_line(line: &str) -> Option<Vec<&str>> {
    // If empty string found return None
    if line.is_empty() {
        return None;
    }

    // Check for tab+//
    // TODO: Refact
    let looking_for = match line.find("\t//") {
        Some(_) => "\t//",
        None => "//",
    };

    // Looking for comments to remove
    match line.find(looking_for) {
        // Once we found comment
        // We remove it and the remaining string
        // is sent to process
        Some(position) => clean_line(&line[0..position]),
        // No comment found,
        // Parse the line by TAB
        None => {
            let vector: Vec<&str> = line
                .split("\t")
                .map(|token| token.trim())
                .collect::<Vec<&str>>();

            // If vector is not empty, returns it
            if vector.len() > 0 {
                return Some(vector);
            }

            // Otherwise return None
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clean_line() {
        assert_eq!(clean_line("1	2	3 //comment"), Some(vec!["1", "2", "3"]));
        assert_eq!(clean_line("4	5	6"), Some(vec!["4", "5", "6"]));
        assert_eq!(clean_line("//comment"), None);
        assert_eq!(
            clean_line("2019-03-11	7	8	9	// comment"),
            Some(vec!["2019-03-11", "7", "8", "9"])
        )
    }
}
