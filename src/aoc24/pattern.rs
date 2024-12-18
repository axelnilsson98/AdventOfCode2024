use std::collections::HashSet;

pub fn pattern() {
    // New set of numbers with duplicates
    let unique_numbers = vec![
        937592051870106, 937592051870106, 937592051870106, 937592051870106,
        937592051870106, 937592051870106, 937592051870106, 813349385513370,
        966181501610394, 958450560543130, 797780128901530, 749401617443226,
        762527037499802, 905706214762906, 905706214762906, 905706214762906,
        905706214762906, 905706214762906, 905706214762906, 905706214762906,
        905706214762906, 905706214762906, 982672028707226, 566813967755674,
        566813967657370, 971434246678938, 1025003226376602, 1032152199276954,
        673711408621978, 982672028543386, 973878083070362, 973878083070362,
        973878083070362, 973878083070362, 973878083070362, 973878083070362,
        973878083070362, 973878083070362, 973878083070362, 628631432047002,
        612136610048410,
    ].into_iter().collect::<HashSet<i64>>().into_iter().collect::<Vec<i64>>();



    // Convert to binary and align by padding
    let binary_representations: Vec<String> = unique_numbers
        .iter()
        .map(|&num| format!("{:b}", num))
        .collect();

    let max_binary_length = binary_representations.iter().map(|s| s.len()).max().unwrap_or(0);
    let padded_binary_representations: Vec<String> = binary_representations
        .iter()
        .map(|s| format!("{:0>width$}", s, width = max_binary_length))
        .collect();

    // Convert binary strings to a 2D array for comparison
    let bit_matrix: Vec<Vec<u8>> = padded_binary_representations
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(2).unwrap() as u8).collect())
        .collect();

    // Determine fixed bits
    let mut fixed_bits = vec![true; max_binary_length];
    for col in 0..max_binary_length {
        let first_bit = bit_matrix[0][col];
        if bit_matrix.iter().any(|row| row[col] != first_bit) {
            fixed_bits[col] = false;
        }
    }

    // Find positions of fixed bits and create the bitwise pattern
    let fixed_positions: Vec<(usize, u8)> = fixed_bits
        .iter()
        .enumerate()
        .filter_map(|(i, &fixed)| if fixed { Some((i, bit_matrix[0][i])) } else { None })
        .collect();

    let bitwise_pattern: String = fixed_bits
        .iter()
        .enumerate()
        .map(|(i, &fixed)| if fixed { bit_matrix[0][i].to_string() } else { "?".to_string() })
        .collect();

    println!("{:?}", fixed_positions);
    println!("{}", bitwise_pattern);
}
