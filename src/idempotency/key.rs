/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-16 21:43:27
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-18 20:19:35
 * @FilePath: \zero2prod\src\idempotency\key.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
#[derive(Debug)]
pub struct IdempotencyKey(String);

impl TryFrom<String> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            anyhow::bail!("The idempotency key cannot be empty");
        }
        let max_length = 50;
        if s.len() >= max_length {
            anyhow::bail!("The idempotency key cannot be shorter than {max_length} characters.");
        }
        Ok(Self(s))
    }
}

impl From<IdempotencyKey> for String {
    fn from(k: IdempotencyKey) -> Self {
        k.0
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
