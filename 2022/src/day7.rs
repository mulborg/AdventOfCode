pub fn day_7(input: &str) {
    println!(
        "Part 1: {}",
        sum_of_total_sizes_of_directories_with_max_size(input, 100000)
    );
}

fn sum_of_total_sizes_of_directories_with_max_size(input: &str, max_size: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day7::sum_of_total_sizes_of_directories_with_max_size;

    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_sum_of_total_sizes_of_directories_with_max_size() {
        assert_eq!(
            sum_of_total_sizes_of_directories_with_max_size(&INPUT.to_string(), 100000),
            95437
        );
    }
}
