extern crate test;

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use crate::Num;

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
        let num1: Num = 9082340701_i64.into();
        let num2: Num = 9082340702_i64.into();
        assert!(num1 < num2);
    }

    #[test]
    fn cmp_test_greaterthen() {
        let num1: Num = 9082340702_i64.into();
        let num2: Num = 9082340701_i64.into();
        assert!(num1 > num2);
    }

    #[test]
    fn cmp_test_eq() {
        let num1: Num = 9082340701_i64.into();
        let num2: Num = 9082340701_i64.into();
        assert!(num1 == num2);
    }

    #[test]
    fn xor() {
        let num1: i64 = 53425433269;
        let num2: i64 = 98675439086;

        let big_num1: Num = num1.into();
        let big_num2: Num = num2.into();

        assert_eq!(num1 ^ num2, (big_num1 ^ big_num2).try_into().unwrap())
    }

    #[test]
    fn and() {
        let num1: i64 = 53425433269;
        let num2: i64 = 98675439086;

        let big_num1: Num = num1.into();
        let big_num2: Num = num2.into();

        assert_eq!(num1 & num2, (big_num1 & big_num2).try_into().unwrap())
    }

    #[test]
    fn or() {
        let num1: i64 = 53425433269;
        let num2: i64 = 98675439086;

        let big_num1: Num = num1.into();
        let big_num2: Num = num2.into();

        assert_eq!(num1 | num2, (big_num1 | big_num2).try_into().unwrap())
    }

    #[test]
    fn not() {
        let num1: i64 = 53425433269;

        let big_num1: Num = num1.into();
        let bug_num1: i64 = (!big_num1).try_into().unwrap();

        //dbg!(&big_num1);

        println!("{:?}", bug_num1);

        assert_eq!(!num1, bug_num1)
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
