use crate::Umpire;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert!(true);
    }

    // #[test]
    // fn test_hungarian_minimize() {
    //     let cost_matrix = vec![
    //         vec![1, 2, 3],
    //         vec![2, 4, 6],
    //         vec![3, 6, 9]
    //     ];
    //     let result = hungarian_minimize(cost_matrix);
    //     assert!(result.is_ok());

    //     let result = result.unwrap();
    //     let expected_result = vec![(0, 0), (1, 1), (2, 2)];
    //     assert_eq!(result, expected_result);
    // }

    #[test]
    fn test_umpire_assign_q1() {
        let mut umpire = Umpire::new(4, 3, 1);
        let result = umpire.assign_game(1, 2);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_ok());
        
        let mut umpire = result.unwrap();
        let result = umpire.assign_game(3, 4);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_ok());

        let mut umpire = result.unwrap();
        let result = umpire.assign_game(1, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_umpire_assign_q2() {
        let mut umpire = Umpire::new(4, 1, 3);
        let result = umpire.assign_game(1, 2);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_ok());

        let mut umpire = result.unwrap();
        let result = umpire.assign_game(3, 4);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_ok());
        
        let mut umpire = result.unwrap();
        let result = umpire.assign_game(1, 4);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_err());
    }
    
    #[test]
    fn test_umpire_unassign() {
        let mut umpire = Umpire::new(4, 2, 1);
        let result = umpire.assign_game(1, 2);
        if let Err(e) = &result {
            println!("Error assign: {}", e);
        }
        assert!(result.is_ok());

        let mut umpire = result.unwrap();
        let result = umpire.unassign_game();
        assert!(result.is_ok());
    }
}