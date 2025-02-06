use chrono::Utc;
use jwt_compact::UntrustedToken;
use tracing::{error, warn};

//无法判断是否有效，但可以判断已过期
pub fn is_jwt_expired(any_jwt: impl AsRef<str>) -> bool {
    if let Ok(token) = UntrustedToken::new(any_jwt.as_ref()) {
        if let Ok(claims) = token.deserialize_claims_unchecked::<()>() {
            if let Some(exp) = claims.expiration {
                return exp <= Utc::now();
            } else {
                warn!("the jwt lacks exp field");
                return false;
            }
        } else {
            error!("can not deserialize the jwt");
        }
    };
    true
}
