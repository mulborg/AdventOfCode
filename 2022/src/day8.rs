pub fn day_8(input: &str) {
    println!("Part 1: {}", count_of_visible_trees(input));
}

fn count_of_visible_trees(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day8::count_of_visible_trees;

    const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_count_of_visible_trees() {
        assert_eq!(count_of_visible_trees(&INPUT.to_string()), 21);
    }
}
