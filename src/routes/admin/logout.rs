/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-15 14:07:36
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 15:17:33
 * @FilePath: \zero2prod\src\routes\admin\logout.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        Ok(see_other("/login"))
    }
}
