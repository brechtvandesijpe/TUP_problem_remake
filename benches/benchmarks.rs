
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
    
    #[bench]
    fn umps10_5_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps10", 5, 2);
            assert_eq!(result, Ok(48942));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps10A_5_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps10A", 5, 2);
            assert_eq!(result, Ok(46551));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps10B_5_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps10B", 5, 2);
            assert_eq!(result, Ok(45609));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps10C_5_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps10C", 5, 2);
            assert_eq!(result, Ok(43149));
            *print.borrow_mut() = Some(result.unwrap());
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps12_7_2(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps12", 7, 2);
            assert_eq!(result, Ok(86889));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps12_6_3(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps12", 6, 3);
            assert_eq!(result, Ok(95259));
            *print.borrow_mut() = Some(result.expect("Failed"));
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps12_5_3(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps12", 5, 3);
            assert_eq!(result, Ok(93679));
            *print.borrow_mut() = Some(result.unwrap());
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
    
    #[bench]
    fn umps12_4_3(b: &mut Bencher) {
        let print: RefCell<Option<i128>> = RefCell::new(None::<i128>);

        b.iter(|| {
            let result: Result<i128, &str> = branch_and_bound("umps12", 4, 3);
            assert_eq!(result, Ok(89826));
            *print.borrow_mut() = Some(result.unwrap());
        });

        eprint!("Result: {:?} ", *print.borrow());
    }
}