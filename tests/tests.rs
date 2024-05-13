#[cfg(test)]
mod tests {
    use TUP_problem_remake::algorithm::Node;
    use TUP_problem_remake::algorithm::Game;

    #[test]
    pub fn is_previous() {
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_one.is_previous(&Game::new(1, 2)));
        assert!(!round_one.is_previous(&Game::new(2, 1)));

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_two.is_previous(&Game::new(3, 4)));
        assert!(!round_two.is_previous(&Game::new(4, 3)));

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_three.is_previous(&Game::new(5, 6)));
        assert!(!round_three.is_previous(&Game::new(6, 5)));

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );
        
        assert!(round_four.is_previous(&Game::new(7, 8)));
        assert!(!round_four.is_previous(&Game::new(8, 7)));
    }

    #[test]
    pub fn previous_constraint() {
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(!round_four.check_previous(&Game::new(1, 2)));
        assert!(round_four.check_previous(&Game::new(2, 1)));
        assert!(!round_four.check_previous(&Game::new(3, 4)));
        assert!(round_four.check_previous(&Game::new(4, 3)));
        assert!(!round_four.check_previous(&Game::new(5, 6)));
        assert!(round_four.check_previous(&Game::new(6, 5)));
        assert!(!round_four.check_previous(&Game::new(7, 8)));
        assert!(round_four.check_previous(&Game::new(8, 7)));
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_one.is_visited(&Game::new(1, 2)));
        assert!(!round_one.is_visited(&Game::new(2, 1)));

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_two.is_visited(&Game::new(3, 4)));
        assert!(!round_two.is_visited(&Game::new(4, 3)));
        assert!(!round_two.is_visited(&Game::new(2, 1)));
        assert!(!round_two.is_visited(&Game::new(1, 2)));

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_three.is_visited(&Game::new(5, 6)));
        assert!(!round_three.is_visited(&Game::new(6, 5)));
        assert!(!round_three.is_visited(&Game::new(3, 4)));
        assert!(!round_three.is_visited(&Game::new(4, 3)));
        assert!(!round_three.is_visited(&Game::new(2, 1)));
        assert!(!round_three.is_visited(&Game::new(1, 2)));

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_four.is_visited(&Game::new(7, 8)));
        assert!(!round_four.is_visited(&Game::new(8, 7)));
        assert!(!round_four.is_visited(&Game::new(5, 6)));
        assert!(!round_four.is_visited(&Game::new(6, 5)));
        assert!(!round_four.is_visited(&Game::new(3, 4)));
        assert!(!round_four.is_visited(&Game::new(4, 3)));
        assert!(!round_four.is_visited(&Game::new(2, 1)));
        assert!(!round_four.is_visited(&Game::new(1, 2)));
    }

    #[test]
    pub fn q1_constraint() {
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_four.check_q1(-1, &Game::new(2, 1)));
        assert!(round_four.check_q1(-1, &Game::new(4, 3)));
        assert!(round_four.check_q1(-1, &Game::new(6, 5)));
        assert!(round_four.check_q1(-1, &Game::new(8, 7)));

        assert!(round_four.check_q1(0, &Game::new(2, 1)));
        assert!(round_four.check_q1(0, &Game::new(4, 3)));
        assert!(round_four.check_q1(0, &Game::new(6, 5)));
        assert!(round_four.check_q1(0, &Game::new(8, 7)));

        assert!(round_four.check_q1(1, &Game::new(2, 1)));
        assert!(round_four.check_q1(2, &Game::new(4, 3)));
        assert!(round_four.check_q1(3, &Game::new(6, 5)));
        assert!(round_four.check_q1(4, &Game::new(8, 7)));

        assert!(!round_four.check_q1(1, &Game::new(1, 2)));
        assert!(!round_four.check_q1(2, &Game::new(3, 4)));
        assert!(!round_four.check_q1(3, &Game::new(5, 6)));
        assert!(!round_four.check_q1(4, &Game::new(7, 8)));
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_one.is_officiated(&Game::new(1, 2)));
        assert!(round_one.is_officiated(&Game::new(2, 1)));
        assert!(!round_one.is_officiated(&Game::new(3, 4)));
        assert!(!round_one.is_officiated(&Game::new(4, 3)));

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_two.is_officiated(&Game::new(3, 4)));
        assert!(round_two.is_officiated(&Game::new(4, 3)));
        assert!(!round_two.is_officiated(&Game::new(2, 1)));
        assert!(!round_two.is_officiated(&Game::new(1, 2)));

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_three.is_officiated(&Game::new(5, 6)));
        assert!(round_three.is_officiated(&Game::new(6, 5)));
        assert!(!round_three.is_officiated(&Game::new(2, 1)));
        assert!(!round_three.is_officiated(&Game::new(1, 2)));

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_four.is_officiated(&Game::new(7, 8)));
        assert!(round_four.is_officiated(&Game::new(8, 7)));
        assert!(!round_four.is_officiated(&Game::new(2, 1)));
        assert!(!round_four.is_officiated(&Game::new(1, 2)));
    }

    #[test]
    pub fn q2_constraint() {
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

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(3, 4);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(5, 6);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(7, 8);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(!round_four.check_q2(-1, &Game::new(1, 2)));
        assert!(!round_four.check_q2(-1, &Game::new(3, 4)));
        assert!(!round_four.check_q2(-1, &Game::new(5, 6)));
        assert!(!round_four.check_q2(-1, &Game::new(7, 8)));

        assert!(!round_four.check_q2(-1, &Game::new(2, 1)));
        assert!(!round_four.check_q2(-1, &Game::new(4, 3)));
        assert!(!round_four.check_q2(-1, &Game::new(6, 5)));
        assert!(!round_four.check_q2(-1, &Game::new(8, 7)));

        assert!(!round_four.check_q2(0, &Game::new(1, 2)));
        assert!(!round_four.check_q2(0, &Game::new(3, 4)));
        assert!(!round_four.check_q2(0, &Game::new(5, 6)));
        assert!(!round_four.check_q2(0, &Game::new(7, 8)));

        assert!(!round_four.check_q2(0, &Game::new(2, 1)));
        assert!(!round_four.check_q2(0, &Game::new(4, 3)));
        assert!(!round_four.check_q2(0, &Game::new(6, 5)));
        assert!(!round_four.check_q2(0, &Game::new(8, 7)));

        assert!(!round_four.check_q2(1, &Game::new(1, 2)));
        assert!(!round_four.check_q2(1, &Game::new(3, 4)));
        assert!(!round_four.check_q2(1, &Game::new(5, 6)));
        assert!(!round_four.check_q2(1, &Game::new(7, 8)));

        assert!(!round_four.check_q2(1, &Game::new(2, 1)));
        assert!(!round_four.check_q2(1, &Game::new(4, 3)));
        assert!(!round_four.check_q2(1, &Game::new(6, 5)));
        assert!(!round_four.check_q2(1, &Game::new(8, 7)));

        assert!(round_four.check_q2(2, &Game::new(1, 2)));
        assert!(!round_four.check_q2(2, &Game::new(3, 4)));
        assert!(!round_four.check_q2(2, &Game::new(5, 6)));
        assert!(!round_four.check_q2(2, &Game::new(7, 8)));

        assert!(round_four.check_q2(2, &Game::new(2, 1)));
        assert!(!round_four.check_q2(2, &Game::new(4, 3)));
        assert!(!round_four.check_q2(2, &Game::new(6, 5)));
        assert!(!round_four.check_q2(2, &Game::new(8, 7)));

        assert!(round_four.check_q2(3, &Game::new(1, 2)));
        assert!(round_four.check_q2(3, &Game::new(3, 4)));
        assert!(!round_four.check_q2(3, &Game::new(5, 6)));
        assert!(!round_four.check_q2(3, &Game::new(7, 8)));

        assert!(round_four.check_q2(3, &Game::new(2, 1)));
        assert!(round_four.check_q2(3, &Game::new(4, 3)));
        assert!(!round_four.check_q2(3, &Game::new(6, 5)));
        assert!(!round_four.check_q2(3, &Game::new(8, 7)));

        assert!(round_four.check_q2(4, &Game::new(1, 2)));
        assert!(round_four.check_q2(4, &Game::new(3, 4)));
        assert!(round_four.check_q2(4, &Game::new(5, 6)));
        assert!(!round_four.check_q2(4, &Game::new(7, 8)));

        assert!(round_four.check_q2(4, &Game::new(2, 1)));
        assert!(round_four.check_q2(4, &Game::new(4, 3)));
        assert!(round_four.check_q2(4, &Game::new(6, 5)));
        assert!(!round_four.check_q2(4, &Game::new(8, 7)));
    }

    #[test]
    pub fn global_constraint() {
        let dist = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];

        let game = Game::new(1, 2);
        let round_one = Node::new(
            None,
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(2, 1);
        let round_two = Node::new(
            Some(Box::new(round_one)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(3, 4);
        let round_three = Node::new(
            Some(Box::new(round_two)),
            &game,
            1,
            &dist,
            0,
        );

        let game = Game::new(4, 3);
        let round_four = Node::new(
            Some(Box::new(round_three)),
            &game,
            1,
            &dist,
            0,
        );

        assert!(round_four.check_global(0));
    }
}