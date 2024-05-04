#[cfg(test)]
mod tests {    
    use TUP_problem_remake::algorithm::Node;

    #[test]
    pub fn sanity() {
        assert!(true);
    }

    #[test]
    pub fn is_visited() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(2, 4), (3, 6), (1, 7), (5, 8)],
            &dist,
        );

        let round_two = Node::new(
            Some(Box::new(round_one.clone())),
            vec![(5, 7), (4, 1), (6, 8), (3, 2)],
            &dist,
        );

        assert!(round_one.is_visited(&vec![(2, 12), (12, 12), (12, 12), (12, 12)]));
        assert!(!round_one.is_visited(&vec![(5, 12), (12, 12), (12, 12), (12, 12)]));
        assert!(round_two.is_visited(&vec![(5, 12), (12, 12), (12, 12), (12, 12)]));
        assert!(!round_two.is_visited(&vec![(2, 12), (12, 12), (12, 12), (12, 12)]));

        assert!(!round_one.is_visited(&vec![(12, 12), (12, 12), (12, 12), (12, 12)]));
        assert!(!round_two.is_visited(&vec![(12, 12), (12, 12), (12, 12), (12, 12)]));
    }

    #[test]
    pub fn q1_constraint_checker_wrong_full() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(2, 4), (3, 6), (1, 7), (5, 8)],
            &dist,
        );

        let round_two = Node::new(
            Some(Box::new(round_one)),
            vec![(5, 7), (4, 1), (6, 8), (3, 2)],
            &dist,
        );

        let round_three = Node::new(
            Some(Box::new(round_two)),
            vec![(6, 3), (8, 7), (2, 1), (4, 5)],
            &dist,
        );

        assert!(!round_three.check_q1(1, &vec![(8, 4), (2, 5), (7, 6), (3, 1)]));
    }

    #[test]
    pub fn q1_constraint_checker_qval() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(1, 5), (4, 8), (6, 2), (7, 3)],
            &dist,
        );

        let round_two = Node::new(
            Some(Box::new(round_one)),
            vec![(2, 8), (5, 3), (4, 7), (1, 6)],
            &dist,
        );

        let round_three = Node::new(
            Some(Box::new(round_two)),
            vec![(5, 6), (1, 4), (3, 8), (2, 7)],
            &dist,
        );

        let round_four = Node::new(
            Some(Box::new(round_three)),
            vec![(3, 7), (8, 2), (5, 4), (6, 1)],
            &dist,
        );

        assert!(!round_four.check_q1(1, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q1(1, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q1(1, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q1(1, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));

        assert!(round_four.check_q1(2, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q1(2, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q1(2, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q1(2, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
        
        assert!(round_four.check_q1(3, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(round_four.check_q1(3, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q1(3, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q1(3, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
        
        assert!(round_four.check_q1(4, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(round_four.check_q1(4, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(round_four.check_q1(4, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q1(4, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
    }

    #[test]
    pub fn is_officiated() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(1, 2), (3, 4), (5, 6), (7, 8)],
            &dist,
        );

        assert!(round_one.is_officiated(&vec![(1, 10), (11, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 2), (11, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (3, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 4), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (5, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 6), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 14), (7, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 14), (15, 8)]));
        
        assert!(round_one.is_officiated(&vec![(9, 1), (11, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(2, 10), (11, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 3), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (4, 12), (13, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 5), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (6, 14), (15, 16)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 14), (15, 7)]));
        assert!(round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 14), (8, 16)]));
        
        assert!(!round_one.is_officiated(&vec![(9, 10), (11, 12), (13, 14), (15, 16)]));
    }

    #[test]
    pub fn q2_constraint_checker_wrong_full() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(2, 4), (3, 6), (1, 7), (5, 8)],
            &dist,
        );

        let round_two = Node::new(
            Some(Box::new(round_one)),
            vec![(5, 7), (4, 1), (6, 8), (3, 2)],
            &dist,
        );

        let round_three = Node::new(
            Some(Box::new(round_two)),
            vec![(6, 3), (8, 7), (2, 1), (4, 5)],
            &dist,
        );

        assert!(!round_three.check_q2(1, &vec![(8, 4), (2, 5), (7, 6), (3, 1)]));
    }

    #[test]
    pub fn q2_constraint_checker_qval() {
        let dist = vec![
            vec![0, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(1, 5), (4, 8), (6, 2), (7, 3)],
            &dist,
        );

        let round_two = Node::new(
            Some(Box::new(round_one)),
            vec![(2, 8), (5, 3), (4, 7), (1, 6)],
            &dist,
        );

        let round_three = Node::new(
            Some(Box::new(round_two)),
            vec![(5, 6), (1, 4), (3, 8), (2, 7)],
            &dist,
        );

        let round_four = Node::new(
            Some(Box::new(round_three)),
            vec![(3, 7), (8, 2), (5, 4), (6, 1)],
            &dist,
        );

        assert!(!round_four.check_q2(1, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q2(1, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q2(1, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q2(1, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));

        assert!(!round_four.check_q2(2, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q2(2, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q2(2, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q2(2, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
        
        assert!(!round_four.check_q2(3, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q2(3, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(!round_four.check_q2(3, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q2(3, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
        
        assert!(!round_four.check_q2(4, &vec![(1, 5), (4, 8), (6, 2), (7, 3)]));
        assert!(!round_four.check_q2(4, &vec![(2, 8), (5, 3), (4, 7), (1, 6)]));
        assert!(round_four.check_q2(4, &vec![(5, 6), (1, 4), (3, 8), (2, 7)]));
        assert!(!round_four.check_q2(4, &vec![(3, 7), (8, 2), (5, 4), (6, 1)]));
    }

    #[test]
    pub fn global_constraint_checker() {
        let dist = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];

        let round_one = Node::new(
            None,
            vec![(1, 1), (1, 1)],
            &dist,
        );

        assert!(round_one.check_global(3));
        assert!(!round_one.check_global(2));
        assert!(!round_one.check_global(1));

        let round_two = Node::new(
            Some(Box::new(round_one)),
            vec![(2, 2), (2, 2)],
            &dist,
        );

        assert!(round_two.check_global(2));
        assert!(!round_two.check_global(1));

        let round_three = Node::new(
            Some(Box::new(round_two)),
            vec![(3, 3), (3, 3)],
            &dist,
        );

        assert!(round_three.check_global(1));

        let round_four = Node::new(
            Some(Box::new(round_three)),
            vec![(4, 4), (4, 4)],
            &dist,
        );

        assert!(round_four.check_global(0));
    }
}