/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-08-20 08:12:35
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2024-12-22 00:36:55
 * @FilePath: \zero2prod\src\routes\mod.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
mod health_check;
mod home;
mod newsletter;
mod subscriptions;
mod subscriptions_confirm;
mod login;

pub use health_check::*;
pub use home::*;
pub use newsletter::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
pub use login::*;
