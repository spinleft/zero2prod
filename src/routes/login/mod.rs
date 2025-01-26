/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-12-22 00:37:03
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2024-12-22 00:52:48
 * @FilePath: \zero2prod\src\routes\login\mod.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved. 
 */
mod get;
mod post;

pub use get::login_form;
pub use post::login;