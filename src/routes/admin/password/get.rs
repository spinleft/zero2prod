/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-14 13:20:44
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 13:32:53
 * @FilePath: \zero2prod\src\routes\admin\password\get.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8"> <title>Change Password</title>
    </head>
<body>
    {msg_html}
    <form action="/admin/password" method="post">
    <label>Current password
        <input
            type="password"
            placeholder="Enter current password"
            name="current_password"
        >
    </label>
    <br>
    <label>New password
        <input
            type="password"
            placeholder="Enter new password"
            name="new_password"
        >
    </label>
    <br>
    <label>Confirm new password
        <input
            type="password"
            placeholder="Type the new password again"
            name="new_password_check"
        >
    </label>
    <br>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>"#,
    )))
}
