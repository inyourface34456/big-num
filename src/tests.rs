extern crate test;

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use crate::Num;
    use rand::Rng;

    #[test]
    fn test_default() {
        let correct = vec![1];
        let mut end = Num::default();
        end.incrment();
        assert_eq!(correct, end.get_data());
    }

    #[test]
    fn test_overflow() {
        let correct = [0, 1]; 
        let mut end = Num::new(&[u8::MAX]);
        end.incrment();
        assert_eq!(correct, end.get_data())
    }

    #[test]
    fn test_overflow_more() {
        let correct = [0, 0, 0, 1];
        let mut end = Num::new(&[u8::MAX; 3]);
        end.incrment();
        assert_eq!(correct, end.get_data())
    }

    #[test]
    fn test_normel_use() {
        let correct = [33, 34, 35];
        let mut end = Num::new(&[32, 34, 35]);
        end.incrment();
        assert_eq!(correct, end.get_data())
    }
    
   #[test]
   fn test_i64() {
        let num: i64 = 9082340701;
        let value: Num = num.into();
        assert_eq!(Ok(num), value.try_into());
    }

    #[test]
    fn test_i64_too_small() {
        let num = [93, 133, 89, 29, 2]; // 9082340701 in bytes
        let value: Num = Num::new(&num);
        assert_eq!(Ok(9082340701_i64), value.try_into());
    }

    #[test]
    fn test_i64_inc() {
        let num = [93, 133, 89, 29, 2]; // 9082340701 in bytes
        let mut value: Num = Num::new(&num);
        value.incrment();
        assert_eq!(Ok(9082340702_i64), value.try_into());
    }

    #[test]
    fn cmp_test_lessthen() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1 = rng.gen_range(0..u128::MAX/2);
            let num2 = rng.gen_range((u128::MAX/2)+1..u128::MAX);

            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();
            assert_eq!(num1 > num2, big_num1 > big_num2, "iter: {i}:\n{:?}\n{:?}", num1, num2);
        }
    }

    #[test]
    fn cmp_test_greaterthen() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1 = rng.gen_range(0..u128::MAX/2);
            let num2 = rng.gen_range((u128::MAX/2)+1..u128::MAX);

            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();
            assert_eq!(num1 < num2, big_num1 < big_num2, "iter: {i}:\n{:?}\n{:?}", num1, num2);
        }
    }

    #[test]
    fn fuzz_cmp_test_eq() {
       let mut rng = rand::thread_rng();
       for _ in 0..1_000_000 {
            let num: i128 = rng.r#gen();
            let big_num: Num = num.into();
            let big_num2: Num = num.into();
            assert!(big_num == big_num2)
       }
    }

    #[test]
    fn fuzz_or() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: i128 = rng.r#gen();
            let num2: i128 = rng.r#gen();
    
            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();

            let res = (big_num1 | big_num2).try_into().unwrap();
            
            assert_eq!(num1 | num2, res, "\niter: {i}: {num1} | {num2} != {res}")
        }
    }

    #[test]
    fn fuzz_not() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: i128 = rng.r#gen();
    
            let big_num1: Num = num1.into();

            let res = (!big_num1).try_into().unwrap();
            
            assert_eq!(!num1, res, "\niter: {i}: !{num1} != {res}")
        }
    }

    #[test]
    fn fuzz_and() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: i128 = rng.r#gen();
            let num2: i128 = rng.r#gen();
    
            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();

            let res = (big_num1 & big_num2).try_into().unwrap();
            
            assert_eq!(num1 & num2, res, "\niter: {i}: {num1} | {num2} != {res}")
        }
    }

    #[test]
    fn fuzz_xor() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: i128 = rng.r#gen();
            let num2: i128 = rng.r#gen();
    
            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();

            let res = (big_num1 ^ big_num2).try_into().unwrap();
            
            assert_eq!(num1 ^ num2, res, "\niter: {i}: {num1} | {num2} != {res}")
        }
    }

    #[test]
    fn fuzz_add() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: u64 = rng.r#gen();
            let num2: u64 = rng.r#gen();
    
            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();

            let res: u128 = (big_num1 + big_num2).try_into().unwrap();
            
            assert_eq!((num1 as u128 + num2 as u128), res, "\niter: {i}\nc: {:?}\ni: {:?}", (num1 as u128 + num2 as u128).to_le_bytes(), res.to_le_bytes())
        }
    }

    #[test]
    fn fuzz_mul() {
        let mut rng = rand::thread_rng();
        for i in 0..1_000_000 {
            let num1: u128 = rng.gen_range(0..(2^64)-1);
            let num2: u128 = rng.gen_range(0..(2^64)-1);
    
            let big_num1: Num = num1.into();
            let big_num2: Num = num2.into();

            let res: u128 = (big_num1 * big_num2).try_into().unwrap();
            
            assert_eq!(num1 * num2, res, "\niter: {i}\nc: {:?}\ni: {:?}", (num1 as u128 * num2 as u128).to_le_bytes(), res.to_le_bytes())
        }
    }

    #[bench]
    fn perf(b: &mut Bencher) -> impl std::process::Termination {
        let mut start = Num::default();
        b.iter(|| start.incrment())
    }

    #[bench]
    fn large_num_inc(b: &mut Bencher) -> impl std::process::Termination {
        let mut start = Num::new(&[126; 1000000]);

        b.iter(|| start.incrment())
    }
}
 