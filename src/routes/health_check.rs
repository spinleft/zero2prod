/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-08-20 08:12:49
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2024-08-20 22:23:59
 * @FilePath: \zero2prod\src\routes\health_check.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved. 
 */
use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
