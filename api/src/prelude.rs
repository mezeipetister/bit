// Copyright (C) 2020 Peter Mezei
//
// This file is part of GNStore.
//
// GNStore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// GNStore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GNStore.  If not, see <http://www.gnu.org/licenses/>.

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::Cursor;

#[derive(Debug)]
// Wrapper for 200 Ok response code
pub struct StatusOk<T>(pub T)
where
    T: Serialize + Debug;
// Wrapper for 201 Created response code
pub struct StatusCreated<T>(pub T)
where
    T: Serialize + Debug;
// Wrapper for 202 Accepted response code
pub struct StatusAccepted<T>(pub T)
where
    T: Serialize + Debug;

// Rocket responder for custom Ok status
impl<'r, T> Responder<'static> for StatusOk<T>
where
    T: Serialize + Debug,
{
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(Cursor::new(serde_json::to_string(&self.0).unwrap()))
            .ok()
    }
}

// Rocket responder for custom Ok status
impl<'r, T> Responder<'static> for StatusCreated<T>
where
    T: Serialize + Debug,
{
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Created)
            .sized_body(Cursor::new(serde_json::to_string(&self.0).unwrap()))
            .ok()
    }
}

// Rocket responder for custom Ok status
impl<'r, T> Responder<'static> for StatusAccepted<T>
where
    T: Serialize + Debug,
{
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Accepted)
            .sized_body(Cursor::new(serde_json::to_string(&self.0).unwrap()))
            .ok()
    }
}

// API Error response scheme
// Use it for all the API error response
#[derive(Serialize, Deserialize, Debug)]
struct ApiErrorScheme {
    message: String,
}

impl ApiErrorScheme {
    fn new(message: String) -> Self {
        ApiErrorScheme { message }
    }
}

// API Error type
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    InternalError(String),
    NotFound,
    Unauthorized,
}

// Rocket responder for ApiError
impl<'r> Responder<'static> for ApiError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .status(match self {
                ApiError::BadRequest(_) => Status::BadRequest,
                ApiError::InternalError(_) => Status::InternalServerError,
                ApiError::NotFound => Status::NotFound,
                ApiError::Unauthorized => Status::Unauthorized,
            })
            .sized_body(Cursor::new(match self {
                ApiError::BadRequest(message) => {
                    serde_json::to_string(&ApiErrorScheme::new(message)).unwrap()
                }
                ApiError::InternalError(message) => {
                    serde_json::to_string(&ApiErrorScheme::new(message)).unwrap()
                }
                ApiError::NotFound => serde_json::to_string(&ApiErrorScheme::new(
                    "A kért oldal nem található".to_owned(),
                ))
                .unwrap(),
                ApiError::Unauthorized => serde_json::to_string(&ApiErrorScheme::new(
                    "Ön nincs bejelentkezve! Jelentkezzen be!".to_owned(),
                ))
                .unwrap(),
            }))
            .ok()
    }
}

// core_lib::Error => ApiError
impl From<core_lib::Error> for ApiError {
    fn from(err: core_lib::Error) -> Self {
        match err {
            core_lib::Error::BadRequest(msg) => ApiError::BadRequest(msg),
            core_lib::Error::InternalError(msg) => ApiError::InternalError(msg),
        }
    }
}

// storaget::PackError => ApiError
impl From<storaget::PackError> for ApiError {
    fn from(err: storaget::PackError) -> Self {
        match err {
            storaget::PackError::DeserializeError(err) => ApiError::InternalError(err),
            storaget::PackError::InternalError(err) => ApiError::InternalError(err),
            storaget::PackError::IOError(err) => ApiError::InternalError(err),
            storaget::PackError::ObjectNotFound => {
                ApiError::InternalError("Adatobjektum nem található a megadott ID-val".to_string())
            }
            storaget::PackError::PathNotFound => {
                ApiError::InternalError("Storage<T> path not found!".to_string())
            }
            storaget::PackError::SerializeError(err) => ApiError::InternalError(err),
            storaget::PackError::IDTaken => {
                ApiError::BadRequest("A megadott ID már foglalt!".to_string())
            }
        }
    }
}
