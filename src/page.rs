use std::ffi::{CStr, CString};
use std::io::Read;
use std::ptr;
use std::slice;
use std::convert::TryFrom;

use mupdf_sys::*;

use crate::{
    context, Buffer, Colorspace, Cookie, Device, DisplayList, Error, Link, LinkDest, LinkDestType, Matrix, Pixmap, Quad,
    Rect, Separations, TextPage, TextPageOptions, document::Location,
};

#[derive(Debug)]
pub struct Page {
    pub(crate) inner: *mut fz_page,
    pub(crate) doc: *mut fz_document,
}

impl Page {
    pub(crate) unsafe fn from_raw(raw: *mut fz_page) -> Self {
        Self {
            inner: raw,
            doc: (*raw).doc,
        }
    }

    pub fn bounds(&self) -> Result<Rect, Error> {
        let rect = unsafe { ffi_try!(mupdf_bound_page(context(), self.inner)) };
        Ok(rect.into())
    }

    pub fn to_pixmap(
        &self,
        ctm: &Matrix,
        cs: &Colorspace,
        alpha: f32,
        show_extras: bool,
    ) -> Result<Pixmap, Error> {
        unsafe {
            let inner = ffi_try!(mupdf_page_to_pixmap(
                context(),
                self.inner,
                ctm.into(),
                cs.inner,
                alpha,
                show_extras
            ));
            Ok(Pixmap::from_raw(inner))
        }
    }

    pub fn to_svg(&self, ctm: &Matrix) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_svg(
                context(),
                self.inner,
                ctm.into(),
                ptr::null_mut()
            ));
            Buffer::from_raw(inner)
        };
        let mut svg = String::new();
        buf.read_to_string(&mut svg)?;
        Ok(svg)
    }

    pub fn to_svg_with_cookie(&self, ctm: &Matrix, cookie: &Cookie) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_svg(
                context(),
                self.inner,
                ctm.into(),
                cookie.inner
            ));
            Buffer::from_raw(inner)
        };
        let mut svg = String::new();
        buf.read_to_string(&mut svg)?;
        Ok(svg)
    }

    pub fn to_text_page(&self, opts: TextPageOptions) -> Result<TextPage, Error> {
        unsafe {
            let inner = ffi_try!(mupdf_page_to_text_page(
                context(),
                self.inner,
                opts.bits() as _
            ));
            Ok(TextPage::from_raw(inner))
        }
    }

    pub fn to_display_list(&self, annotations: bool) -> Result<DisplayList, Error> {
        unsafe {
            let inner = ffi_try!(mupdf_page_to_display_list(
                context(),
                self.inner,
                annotations
            ));
            Ok(DisplayList::from_raw(inner))
        }
    }

    pub fn run(&self, device: &Device, ctm: &Matrix) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                ptr::null_mut()
            ))
        }
        Ok(())
    }

    pub fn run_with_cookie(
        &self,
        device: &Device,
        ctm: &Matrix,
        cookie: &Cookie,
    ) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                cookie.inner
            ))
        }
        Ok(())
    }

    pub fn run_contents(&self, device: &Device, ctm: &Matrix) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_contents(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                ptr::null_mut()
            ))
        }
        Ok(())
    }

    pub fn run_contents_with_cookie(
        &self,
        device: &Device,
        ctm: &Matrix,
        cookie: &Cookie,
    ) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_contents(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                cookie.inner
            ))
        }
        Ok(())
    }

    pub fn run_annotations(&self, device: &Device, ctm: &Matrix) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_annots(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                ptr::null_mut()
            ))
        }
        Ok(())
    }

    pub fn run_annotations_with_cookie(
        &self,
        device: &Device,
        ctm: &Matrix,
        cookie: &Cookie,
    ) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_annots(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                cookie.inner
            ))
        }
        Ok(())
    }

    pub fn run_widgets(&self, device: &Device, ctm: &Matrix) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_widgets(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                ptr::null_mut()
            ))
        }
        Ok(())
    }

    pub fn run_widgets_with_cookie(
        &self,
        device: &Device,
        ctm: &Matrix,
        cookie: &Cookie,
    ) -> Result<(), Error> {
        unsafe {
            ffi_try!(mupdf_run_page_widgets(
                context(),
                self.inner,
                device.dev,
                ctm.into(),
                cookie.inner
            ))
        }
        Ok(())
    }

    pub fn to_html(&self) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_html(context(), self.inner));
            Buffer::from_raw(inner)
        };
        let mut out = String::new();
        buf.read_to_string(&mut out)?;
        Ok(out)
    }

    pub fn to_xhtml(&self) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_xhtml(context(), self.inner));
            Buffer::from_raw(inner)
        };
        let mut out = String::new();
        buf.read_to_string(&mut out)?;
        Ok(out)
    }

    pub fn to_xml(&self) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_xml(context(), self.inner));
            Buffer::from_raw(inner)
        };
        let mut out = String::new();
        buf.read_to_string(&mut out)?;
        Ok(out)
    }

    pub fn to_text(&self) -> Result<String, Error> {
        let mut buf = unsafe {
            let inner = ffi_try!(mupdf_page_to_text(context(), self.inner));
            Buffer::from_raw(inner)
        };
        let mut out = String::new();
        buf.read_to_string(&mut out)?;
        Ok(out)
    }

    pub fn links(&self) -> Result<LinkIter, Error> {
        let next = unsafe { ffi_try!(mupdf_load_links(context(), self.inner)) };
        Ok(LinkIter {
            next,
            doc: self.doc,
        })
    }

    pub fn separations(&self) -> Result<Separations, Error> {
        unsafe {
            let inner = ffi_try!(mupdf_page_separations(context(), self.inner));
            Ok(Separations::from_raw(inner))
        }
    }

    pub fn search(&self, needle: &str, hit_max: u32) -> Result<Vec<Quad>, Error> {
        struct Quads(*mut fz_quad);

        impl Drop for Quads {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe { fz_free(context(), self.0 as _) };
                }
            }
        }

        let c_needle = CString::new(needle)?;
        let hit_max = if hit_max < 1 { 16 } else { hit_max };
        let mut hit_count = 0;
        unsafe {
            let quads = Quads(ffi_try!(mupdf_search_page(
                context(),
                self.inner,
                c_needle.as_ptr(),
                hit_max as _,
                &mut hit_count
            )));
            if hit_count == 0 {
                return Ok(Vec::new());
            }
            let items = slice::from_raw_parts(quads.0, hit_count as usize);
            Ok(items.iter().map(|quad| (*quad).into()).collect())
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                fz_drop_page(context(), self.inner);
            }
        }
    }
}

#[derive(Debug)]
pub struct LinkIter {
    next: *mut fz_link,
    doc: *mut fz_document,
}

impl Iterator for LinkIter {
    type Item = Link;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        let node = self.next;
        unsafe {
            self.next = (*node).next;
            let bounds = (*node).rect.into();
            let uri = CStr::from_ptr((*node).uri).to_string_lossy().into_owned();
            let mut dest = None;
            let page = 0;
            if fz_is_external_link(context(), (*node).uri) == 0 {
                let fz_dest = fz_resolve_link_dest(
                    context(),
                    self.doc,
                    (*node).uri,
                );
                dest = Some(LinkDest {
                    location: Location {
                        chapter: fz_dest.loc.chapter,
                        page: fz_dest.loc.page,
                    },
                    dest_type: LinkDestType::try_from(fz_dest.type_).ok()?,
                    x: fz_dest.x,
                    y: fz_dest.y,
                    w: fz_dest.w,
                    h: fz_dest.h,
                    zoom: fz_dest.zoom,
                });
            }
            Some(Link {
                bounds,
                dest,
                page,
                uri,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Document, Matrix};

    #[test]
    fn test_page_to_svg() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let svg = page0.to_svg(&Matrix::IDENTITY).unwrap();
        assert!(!svg.is_empty());
    }

    #[test]
    fn test_page_to_html() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let html = page0.to_html().unwrap();
        assert!(!html.is_empty());
    }

    #[test]
    fn test_page_to_xhtml() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let xhtml = page0.to_xhtml().unwrap();
        assert!(!xhtml.is_empty());
    }

    #[test]
    fn test_page_to_xml() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let xml = page0.to_xml().unwrap();
        assert!(!xml.is_empty());
    }

    #[test]
    fn test_page_to_text() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let text = page0.to_text().unwrap();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_page_to_display_list() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let _dl = page0.to_display_list(true).unwrap();
        let _dl = page0.to_display_list(false).unwrap();
    }

    #[test]
    fn test_page_to_text_page() {
        use crate::TextPageOptions;

        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let _tp = page0
            .to_text_page(TextPageOptions::PRESERVE_IMAGES)
            .unwrap();
    }

    #[test]
    fn test_page_links() {
        use crate::Link;

        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let links_iter = page0.links().unwrap();
        let links: Vec<Link> = links_iter.collect();
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn test_page_separations() {
        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let seps = page0.separations().unwrap();
        assert_eq!(seps.len(), 0);
    }

    #[test]
    fn test_page_search() {
        use crate::{Point, Quad};

        let doc = Document::open("tests/files/dummy.pdf").unwrap();
        let page0 = doc.load_page(0).unwrap();
        let hits = page0.search("Dummy", 1).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(
            hits,
            [Quad {
                ul: Point {
                    x: 56.8,
                    y: 69.32512
                },
                ur: Point {
                    x: 115.85405,
                    y: 69.32512
                },
                ll: Point {
                    x: 56.8,
                    y: 87.311844
                },
                lr: Point {
                    x: 115.85405,
                    y: 87.311844
                }
            }]
        );

        let hits = page0.search("Not Found", 1).unwrap();
        assert_eq!(hits.len(), 0);
    }
}
