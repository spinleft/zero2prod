/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-15 19:04:35
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 20:03:08
 * @FilePath: \zero2prod\src\routes\admin\newsletter\mod.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
mod get;
mod post;

pub use get::publish_newsletter_form;
pub use post::publish_newsletter;
