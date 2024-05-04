#![feature(test)]
extern crate test;

mod benchs {
    use test::Bencher;
    use TUP_problem_remake::algorithm::branch_and_bound; // Replace with your actual crate name
    
    #[bench]
    fn umps8_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps8", 4, 2));
    }
    
    #[bench]
    fn umps8A_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps8A", 4, 2));
    }
    
    #[bench]
    fn umps8B_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps8B", 4, 2));
    }
    
    #[bench]
    fn umps8C_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps8C", 4, 2));
    }
    
    #[bench]
    fn umps10_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps10", 4, 2));
    }
    
    #[bench]
    fn umps10A_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps10A", 4, 2));
    }
    
    #[bench]
    fn umps10B_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps10B", 4, 2));
    }
    
    #[bench]
    fn umps10C_4_2(b: &mut Bencher) {
        b.iter(|| branch_and_bound("umps10C", 4, 2));
    }
}