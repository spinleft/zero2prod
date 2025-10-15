/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2025-10-14 01:46:49
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2025-10-15 15:18:40
 * @FilePath: \zero2prod\src\session_state.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
use actix_session::SessionExt;
use actix_session::{Session, SessionGetError, SessionInsertError};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn log_out(&self) {
        self.0.purge();
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();
        ready(Ok(TypedSession(session)))
    }
}
