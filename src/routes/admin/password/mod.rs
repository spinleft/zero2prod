/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-14 13:12:48
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 15:19:29
 * @FilePath: \zero2prod\src\routes\admin\password\mod.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
mod get;
pub use get::change_password_form;
mod post;
pub use post::change_password;
