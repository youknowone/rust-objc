#![feature(macro_rules)]

extern crate libc;
mod objc;

struct Crawler {
    url: objc::id,
    data: objc::id,
}

impl Crawler {
    pub fn with_url(s: &str) -> Crawler {
        let url = send![send![Class!(NSURL) URLWithString:objc_string!(s)] retain];
        Crawler { url: url, data: objc::nil }
    }

    pub fn crawl(&mut self) -> objc::id {
        NSLog!("saved url: %@", self.url);
        self.data = send![send![Class!(NSData) dataWithContentsOfURL:self.url] retain];
        return self.data
    }

    pub fn print(&self) {
        let string = send![send![Class!(NSString) alloc] initWithData:self.data encoding:objc::NSUTF8StringEncoding];
        assert!(string != objc::nil);
        unsafe { objc::NSLog(objc_string!("%@"), string) };
        NSLog!("%@", string);
        send![string release];
    }
}

pub fn main() {
    let mut crawler = Crawler::with_url("http://rust-kr.org/");
    crawler.crawl();
    crawler.print();
}