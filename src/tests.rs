#[cfg(test)]
mod test {
    use crate::calculator::RpnCalculator;

    #[test]
    fn calc_result_ok(){
        let calc = RpnCalculator::new(false);

        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    #[test]
    #[should_panic]
    fn panic_pattern() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
        calc.eval("1 1 + - ");
    }
}

