/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-15 14:41:36
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 15:17:19
 * @FilePath: \zero2prod\src\authentication\mod.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
mod middleware;
mod password;
pub use middleware::reject_anonymous_users;
pub use middleware::UserId;
pub use password::{change_password, validate_credentials, AuthError, Credentials};
