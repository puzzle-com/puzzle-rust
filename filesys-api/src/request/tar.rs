// Copyright 2017 rust-filesys-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use http::Method;
use request::ApiRequest;

pub struct TarAdd;

impl_skip_serialize!(TarAdd);

impl ApiRequest for TarAdd {
    const PATH: &'static str = "/tar/add";

    const METHOD: &'static Method = &Method::POST;
}

#[derive(Serialize)]
pub struct TarCat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for TarCat<'a> {
    const PATH: &'static str = "/tar/cat";
}
