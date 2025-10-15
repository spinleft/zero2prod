/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-12-22 00:37:32
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 15:18:59
 * @FilePath: \zero2prod\src\routes\login\get.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-12-22 00:37:32
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 14:21:47
 * @FilePath: \zero2prod\src\routes\login\get.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            >
        </label>
        <button type="submit">Login</button> </form>
</body>
</html>"#,
        ))
}
