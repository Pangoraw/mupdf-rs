use std::fmt;

use mupdf_sys::*;
use num_enum::TryFromPrimitive;

use crate::{Rect, document::Location};

#[derive(Debug, Clone, TryFromPrimitive)]
#[repr(u32)]
pub enum LinkDestType {
    LINK_DEST_FIT,
	LINK_DEST_FIT_B,
	LINK_DEST_FIT_H,
	LINK_DEST_FIT_BH,
	LINK_DEST_FIT_V,
	LINK_DEST_FIT_BV,
	LINK_DEST_FIT_R,
	LINK_DEST_XYZ,
}

#[derive(Debug, Clone)]
pub struct LinkDest {
    pub location: Location,
    pub dest_type: LinkDestType,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub zoom: f32,
}

/// A list of interactive links on a page.
#[derive(Debug, Clone)]
pub struct Link {
    pub bounds: Rect,
    pub dest: Option<LinkDest>,
    pub page: u32,
    pub uri: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Link(b={},page={},uri={})",
            self.bounds, self.page, self.uri
        )
    }
}
