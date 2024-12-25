#[cfg(test)]
mod tests {
    use crate::vault::pool_factory::*;
    use crate::utils::types::*;

    #[tokio::test]
    async fn test_create_pools_success() {
        let pool_data = vec!["token1".to_string(), "token2".to_string()];
        let result = create_pools(pool_data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_pools_empty_data() {
        let pool_data: Vec<String> = vec![];
        let result = create_pools(pool_data).await;
        assert_eq!(result, Err(CustomError::PoolDataEmpty));
    }

    #[tokio::test]
    async fn test_create_pools_another_operation_in_progress() {
        let pool_data = vec!["token1".to_string()];
        {
            let mut locks = crate::vault::LOCKS.lock().unwrap();
            locks.insert("token1".to_string(), true);
        }

        let result = create_pools(pool_data).await;
        assert_eq!(
            result,
            Err(CustomError::AnotherOperationInProgress("token1".to_string()))
        );
    }

    #[tokio::test]
    async fn test_create_pools_token_deposit_failed() {
        let pool_data = vec!["fail".to_string()];
        let result = create_pools(pool_data).await;
        assert_eq!(result, Err(CustomError::TokenDepositFailed));
    }
}
