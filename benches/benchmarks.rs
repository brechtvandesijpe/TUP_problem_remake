
#![feature(test)]
extern crate test;

mod benchs {
    use std::cell::RefCell;
    use test::Bencher;
    use TUP_problem_remake::algorithm::branch_and_bound; // Replace with your actual crate name
    
    #[bench]
    fn umps8_4_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps8", 4, 2);
            assert_eq!(result, Ok(34311));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps8A_4_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result = branch_and_bound("umps8A", 4, 2);
            assert_eq!(result, Ok(31490));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps8B_4_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps8B", 4, 2);
            assert_eq!(result, Ok(32731));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps8C_4_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps8C", 4, 2);
            assert_eq!(result, Ok(29879));
            *print.borrow_mut() = Some(result.unwrap());
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
}