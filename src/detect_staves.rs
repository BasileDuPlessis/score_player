fn detect_lines(pixels: &Vec<Vec<u16>>) -> Vec<bool> {
    let mut result = Vec::new();
    let mut row_value: Vec<f32> = Vec::new();

    for line in pixels {
        row_value.push(mean(line));
    }

    let row_value_mean = mean(&row_value);

    let row_value_deviation = standard_deviation(&row_value, row_value_mean);

    let threshold = row_value_mean - row_value_deviation;

    for v in row_value.iter() {
        result.push(v <= &threshold);
    }

    result
}

fn mean<T>(numbers: &Vec<T>) -> f32
where
    T: std::convert::Into<f32>,
    T: Copy,
{
    let mut sum = 0.0;

    for n in numbers {
        sum += Into::<f32>::into(*n);
    }

    sum / numbers.len() as f32
}

fn standard_deviation(numbers: &Vec<f32>, mean: f32) -> f32 {
    let mut variance = 0.0;
    for n in numbers.iter() {
        variance += (n - mean).powi(2);
    }
    variance /= (numbers.len() - 1) as f32;

    variance.sqrt()
}

fn group_lines(black_line_per_rows: &Vec<bool>) -> Vec<(usize, usize)> {
    let mut black_lines: Vec<(usize, usize)> = Vec::new();
    let mut iter = black_line_per_rows.iter().enumerate();

    while let Some((index, is_line)) = iter.next() {
        if *is_line {
            let mut line_thickness = 1;
            let line_begin_at_row = index;

            'get_line_thickness: while let Some((_, is_line)) = iter.next() {
                if !*is_line {
                    break 'get_line_thickness;
                }
                line_thickness += 1;
            }

            black_lines.push((line_begin_at_row, line_thickness));
        }
    }

    black_lines
}

fn is_staff(black_lines: &[(usize, usize); 5]) -> bool {
    let mut min_line_thickness = 0;
    let mut max_line_thickness = 0;
    let mut min_spacing_thickness = 0;
    let mut max_spacing_thickness = 0;

    for w in black_lines.windows(2) {
        let line_thickness = w[0].1;
        let spacing_thickness = w[1].0 - (w[0].0 + w[1].1);

        min_line_thickness = get_min(min_line_thickness, line_thickness);
        min_spacing_thickness = get_min(min_spacing_thickness, spacing_thickness);

        max_line_thickness = get_max(max_line_thickness, line_thickness);
        max_spacing_thickness = get_max(max_spacing_thickness, spacing_thickness);
    }

    fn get_min(reference: usize, value: usize) -> usize {
        if value < reference || reference == 0 {
            value
        } else {
            reference
        }
    }

    fn get_max(reference: usize, value: usize) -> usize {
        if value > reference {
            value
        } else {
            reference
        }
    }

    let deviation = min_line_thickness;

    max_line_thickness - min_line_thickness <= deviation
        && max_spacing_thickness - min_spacing_thickness <= deviation
}

#[cfg(test)]
mod tests {

    #[test]
    fn pixels_to_lines() {
        assert_eq!(
            super::detect_lines(&vec![vec![1, 1], vec![0, 1], vec![1, 1],]),
            vec![false, true, false]
        );
        assert_eq!(
            super::detect_lines(&vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
            ]),
            vec![false, true, false, true, false, true, false, true, false]
        );
    }

    #[test]
    fn mean() {
        assert_eq!(super::mean(&vec![0u16]), 0.);
        assert_eq!(super::mean(&vec![1u16, 2u16]), 1.5);
    }

    #[test]
    fn standard_deviation() {
        assert_eq!(super::standard_deviation(&vec![1., 1.], 1.), 0.);
        assert_eq!(super::standard_deviation(&vec![2., 4., 6.], 4.), 2.);
    }

    #[test]
    fn black_lines() {
        group_lines(
            &vec![true], //on line
            vec![(0, 1)],
        );
        group_lines(
            &vec![true, true], //one bold line
            vec![(0, 2)],
        );
        group_lines(
            &vec![false, true], //one line after spacing
            vec![(1, 1)],
        );
        group_lines(
            &vec![true, false, true], //two lines
            vec![(0, 1), (2, 1)],
        );
    }

    fn group_lines(black_line_per_rows: &Vec<bool>, black_lines: Vec<(usize, usize)>) {
        assert_eq!(black_lines, super::group_lines(black_line_per_rows))
    }

    #[test]
    fn staves() {
        is_staff(&[(0, 1), (4, 1), (8, 1), (12, 1), (16, 1)]); //regular
        is_staff(&[(0, 1), (4, 2), (8, 1), (12, 1), (16, 1)]); //light variations
    }

    fn is_staff(staff_line_index: &[(usize, usize); 5]) {
        assert!(super::is_staff(staff_line_index));
    }
}
