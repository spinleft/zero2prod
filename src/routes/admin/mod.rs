/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-13 22:15:27
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-14 17:02:37
 * @FilePath: \zero2prod\src\routes\admin\mod.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
mod dashboard;
mod logout;
mod password;

pub use dashboard::admin_dashboard;
pub use logout::log_out;
pub use password::change_password;
pub use password::change_password_form;
