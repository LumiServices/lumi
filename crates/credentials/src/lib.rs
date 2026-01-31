pub mod credentials;
pub const DEFAULT_ACCESS_KEY: &str = "lumiserver";
pub const DEFAULT_SECRET_KEY: &str = "lumiserver";
pub const MIN_LEG_ACCESS_KEY: usize = 6;
pub const MAX_LEG_ACCESS_KEY: usize = 20; //ill probably remove this later tbh
pub const MIN_LEG_SECRET_KEY: usize = 6;
pub const MAX_LEG_SECRET_KEY: usize = 40; //same with this
pub const ALPHA_NUMERIC_TABLE: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[cfg(test)]
mod test {
    #[test]
    fn test_generate_access_key_valid_length() {
        let key = crate::credentials::generate_access_key(20).unwrap();
        assert_eq!(key.len(), 20);
    }
    #[test]
       fn test_generate_access_key_too_short() {
        let result = crate::credentials::generate_access_key(crate::MIN_LEG_ACCESS_KEY - 1);
        assert!(matches!(result, Err(crate::credentials::KeyError::AccessKeyTooShort)));
    }
}