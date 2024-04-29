#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data() {
        let data = read_data("resources/umps8.txt");
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.n_teams, /* expected number of teams */);
        assert_eq!(data.dist, /* expected dist */);
        assert_eq!(data.opponents, /* expected opponents */);
    }

    #[test]
    fn test_model_initialisation() {
        let data = read_data("resources/umps8.txt");
        let data = data.unwrap();
        let model = Model::new(data);
        assert!(model.is_ok());
    }

    #[test]
    fn test_hungarian_minimize() {
        let cost_matrix = vec![
            vec![1, 2, 3],
            vec![2, 4, 6],
            vec![3, 6, 9]
        ];
        let result = hungarian_minimize(cost_matrix);
        assert!(result.is_ok());

        let result = result.unwrap();
        let expected_result = vec![(0, 0), (1, 1), (2, 2)];
        assert_eq!(result, expected_result);
    }
}