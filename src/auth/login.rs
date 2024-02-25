use axum::Extension;
use axum::response::{Redirect};
use crate::OAuthInfo;

pub(crate) async fn login(
    Extension(oauth_info): Extension<OAuthInfo>
) -> Redirect {
    Redirect::to(
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={}&response_type=code&redirect_uri={}",
            oauth_info.oauth_id,
            oauth_info.oauth_redirect_uri
        )
            .as_str()
    )
}
