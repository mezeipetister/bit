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

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        // if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        // {
        //     // Allow API requests from anywhere
        //     response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        //     response.set_header(Header::new(
        //         "Access-Control-Allow-Methods",
        //         "POST, GET, OPTIONS, PUT, DELETE",
        //     ));
        //     response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        //     response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        // }

        // if request.method() == Method::Options {

        //     response.set_header(ContentType::Plain);
        //     response.set_sized_body(Cursor::new(""));
        // }
    }
}
