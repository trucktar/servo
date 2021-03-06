/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(unsafe_code)]

use crate::data::StyleAndLayoutData;
use script_layout_interface::wrapper_traits::GetLayoutData;

pub trait GetRawData<'dom> {
    fn get_raw_data(self) -> Option<&'dom StyleAndLayoutData>;
}

impl<'dom, T> GetRawData<'dom> for T
where
    T: GetLayoutData<'dom>,
{
    fn get_raw_data(self) -> Option<&'dom StyleAndLayoutData> {
        self.get_style_and_layout_data()
            .map(|opaque| opaque.downcast_ref().unwrap())
    }
}
