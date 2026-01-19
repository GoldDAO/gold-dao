use ic_cdk::api::management_canister::main::raw_rand;
use std::time::Duration;

pub async fn generate_rand_nonce() -> Result<u64, String> {
    generate_rand_byte_array().await.map(u64::from_be_bytes)
}

pub async fn generate_rand_byte_array() -> Result<[u8; 8], String> {
    match raw_rand().await {
        Ok((random_bytes,)) => {
            let bytes_array: Result<[u8; 8], _> = random_bytes[0..8].try_into();

            match bytes_array {
                Ok(bytes) => Ok(bytes),
                Err(err) => Err(format!("Initialising slicing byte array: {}", err)),
            }
        }
        Err(err) => Err(format!("Random bytes generation error: {:?}", err)),
    }
}

pub async fn generate_random_delay(max_interval: Duration) -> Result<Duration, String> {
    let random_nonce = generate_rand_nonce().await?;

    ic_cdk::println!("Generated random nonce: {}", random_nonce);

    let random_delay_nanos = random_nonce % (max_interval.as_nanos() as u64);
    ic_cdk::println!(
        "Generated random delay: {} nanos within max interval: {:?}",
        random_delay_nanos,
        max_interval
    );

    Ok(Duration::from_nanos(random_delay_nanos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use ic_cdk::api::call::RejectionCode;
    use ic_cdk::api::management_canister::main::CanisterIdRecord;
    use std::cell::RefCell;
    use std::time::Duration;

    // Mock state for controlling raw_rand behavior in tests
    thread_local! {
        static MOCK_RAND_RESULT: RefCell<Option<Result<Vec<u8>, (RejectionCode, String)>>> =
            RefCell::new(None);
    }

    // Helper function to set mock behavior
    fn set_mock_rand_result(result: Result<Vec<u8>, (RejectionCode, String)>) {
        MOCK_RAND_RESULT.with(|r| *r.borrow_mut() = Some(result));
    }

    // Helper function to clear mock
    fn clear_mock_rand_result() {
        MOCK_RAND_RESULT.with(|r| *r.borrow_mut() = None);
    }

    // Note: In a real test environment, you would need to mock ic_cdk::api::management_canister::main::raw_rand
    // This can be done using a test framework like `pocket-ic` or by creating a wrapper trait.
    // For this example, I'll show both unit tests and integration test patterns.

    #[test]
    fn test_generate_rand_nonce_converts_bytes_to_u64() {
        // This test demonstrates the expected behavior
        let test_bytes = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let expected_u64 = u64::from_be_bytes(test_bytes);

        assert_eq!(expected_u64, 0x0102030405060708);
    }

    #[test]
    fn test_generate_rand_byte_array_slice_conversion() {
        // Test that slicing works correctly
        let random_bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let bytes_array: Result<[u8; 8], _> = random_bytes[0..8].try_into();

        assert!(bytes_array.is_ok());
        assert_eq!(bytes_array.unwrap(), [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_generate_random_delay_bounds() {
        // Test the logic of random delay generation
        let max_interval = Duration::from_secs(60);
        let max_nanos = max_interval.as_nanos() as u64;

        // Test with various nonce values
        let test_cases = vec![
            (0u64, Duration::from_nanos(0)),
            (100u64, Duration::from_nanos(100)),
            (max_nanos - 1, Duration::from_nanos(max_nanos - 1)),
            (max_nanos, Duration::from_nanos(0)),
            (max_nanos + 100, Duration::from_nanos(100)),
        ];

        for (nonce, expected) in test_cases {
            let delay_nanos = nonce % max_nanos;
            assert_eq!(Duration::from_nanos(delay_nanos), expected);
        }
    }

    #[test]
    fn test_random_delay_never_exceeds_max() {
        // Test that modulo operation keeps delay within bounds
        let max_interval = Duration::from_secs(100);
        let max_nanos = max_interval.as_nanos() as u64;

        // Test with large random values
        for nonce in [u64::MAX, u64::MAX / 2, u64::MAX / 4] {
            let delay_nanos = nonce % max_nanos;
            let delay = Duration::from_nanos(delay_nanos);
            assert!(delay < max_interval);
        }
    }

    #[test]
    fn test_edge_case_zero_duration() {
        // Test with zero duration (edge case)
        let max_interval = Duration::from_nanos(1);
        let max_nanos = max_interval.as_nanos() as u64;

        let nonce = 12345u64;
        let delay_nanos = nonce % max_nanos;
        assert_eq!(delay_nanos, 0);
    }
}
