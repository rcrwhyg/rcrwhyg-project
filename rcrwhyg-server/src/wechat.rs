use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenResponse {
    pub access_token: Option<String>,
    pub expires_in: Option<i32>,
    pub err_code: Option<i32>,
    pub err_msg: Option<String>,
}

pub async fn get_wechat_access_token(app_id: &str, app_secret: &str) -> Result<String, String> {
    let url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        app_id, app_secret
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let token_response: AccessTokenResponse = response.json().await.map_err(|e| e.to_string())?;

    println!("微信接口返回: {:?}", token_response);

    if let Some(token) = token_response.access_token {
        Ok(token)
    } else {
        Err(format!(
            "获取Token失败: code {:?}, msg {:?}",
            token_response.err_code, token_response.err_msg
        ))
    }
}
