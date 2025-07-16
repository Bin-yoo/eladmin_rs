use captcha_rs::{Captcha, CaptchaBuilder};
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginProperties {
    /// 是否启用登录缓存
    #[serde(rename = "cache-enable")]
    pub cache_enable: bool,

    /// 是否限制单用户登录
    pub single: bool,

    /// 验证码相关配置
    #[serde(rename = "login-code")]
    pub login_code: LoginCodeConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginCodeConfig {
    /// 是否启用验证码
    pub enabled: bool,

    /// 验证码类型配置（如 "spec", "arithmetic" 等）
    #[serde(rename = "code-type")]
    pub code_type: LoginCodeType,

    /// 登录图形验证码有效时间（单位：分钟）
    pub expiration: i64,

    /// 验证码高度
    pub height: u32,

    /// 验证码宽度
    pub width: u32,

    /// 内容长度
    pub length: usize,

    /// 字体名称，为空则使用默认字体
    #[serde(rename = "font-name")]
    pub font_name: Option<String>,

    /// 字体大小
    #[serde(rename = "font-size")]
    pub font_size: u32,
}

impl LoginCodeConfig {
    pub fn switch_captcha(&self) -> Captcha {
        let captcha = match self.code_type {
            // LoginCodeType::Chinese => todo!(),
            // LoginCodeType::ChineseGif => todo!(),
            // LoginCodeType::Gif => todo!(),
            // LoginCodeType::Spec => todo!(),
            // LoginCodeType::Arithmetic => todo!(),
            _ => CaptchaBuilder::new()
                    .length(self.length)
                    .width(self.width)
                    .height(self.height)
                    .dark_mode(false)
                    .complexity(3) // min: 1, max: 10
                    .compression(40) // min: 1, max: 99
                    .build()
        };
        // 设置字体（如果有配置）
        // todo!("set font");
        
        captcha
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum LoginCodeType {
    Chinese,
    ChineseGif,
    Gif,
    Spec,
    Arithmetic,
}

impl<'a> Deserialize<'a> for LoginCodeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct LoginCodeTypeVisitor;

        impl<'a> Visitor<'a> for LoginCodeTypeVisitor {
            type Value = LoginCodeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing the login code type")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "chinese" => Ok(LoginCodeType::Chinese),
                    "chinese_gif" => Ok(LoginCodeType::ChineseGif),
                    "gif" => Ok(LoginCodeType::Gif),
                    "spec" => Ok(LoginCodeType::Spec),
                    "arithmetic" => Ok(LoginCodeType::Arithmetic),
                    _ => Err(E::custom(format!("unknown login code type: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(LoginCodeTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use captcha_rs::CaptchaBuilder;

    #[test]
    fn test_generate_captcha() {
        let captcha = CaptchaBuilder::new()
            .length(4)
            .width(111)
            .height(36)
            .dark_mode(false)
            .complexity(3) // min: 1, max: 10
            .compression(40) // min: 1, max: 99
            .build();
        
        println!("text: {}", captcha.text);
        println!("base_img: {}", captcha.to_base64());
    }

}