use super::*;

// ------------------- COMPENSATION_FACTOR -------------------
#[test]
fn test_get_compensation_factor_b1() {
    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
}

#[test]
fn test_set_compensation_factor_b1() {
    let _ = set_compensation_factor(5);
    let factor = get_compensation_factor();

    assert_eq!(factor, 5);
}
#[test]
fn test_set_compensation_factor_b2() {
    let res = set_compensation_factor(15);

    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
    assert_eq!(
        res,
        Err(
            CustomError::new_with_message(
                ErrorType::Other,
                "Compensation factor value has to be between (including) 1 and 10 (mean 0.1% and 1%).".to_string()
            )
        )
    );
}
#[test]
fn test_set_compensation_factor_b3() {
    let res = set_compensation_factor(0);

    let factor = get_compensation_factor();

    assert_eq!(factor, 10);
    assert_eq!(
        res,
        Err(
            CustomError::new_with_message(
                ErrorType::Other,
                "Compensation factor value has to be between (including) 1 and 10 (mean 0.1% and 1%).".to_string()
            )
        )
    );
}
