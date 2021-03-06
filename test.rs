
#![feature(libc)]
extern crate libc;
#[macro_use]
extern crate objc;

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use libc;
    use objc;

    #[test]
    pub fn test_c_str() {
        let str = "test c_str end";
        let c_str = c_str!(str);
        let len = unsafe {
            libc::strlen(c_str)
        } as usize;
        assert!(str.len() == len);
    }

    #[test]
    pub fn test_class() {
        let NSObject = unsafe { Class!(NSObject) };
        assert!(NSObject != objc::Nil);
    }

    #[test]
    pub fn test_selector() {
        unsafe {
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = objc::NSStringFromSelector(selector);
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_send() {
        unsafe {
            let NSString = Class!(NSString);
            assert!(NSString != objc::Nil);
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = send![NSString stringWithUTF8String:c_str!("test string end")];
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_send2() {
        unsafe {
            let NSString = Class!(NSString);
            assert!(NSString != objc::Nil);
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = send![NSString stringWithUTF8String:c_str!("test string end")];
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_string() {
        unsafe {
            let NSString = Class!(NSString);
            let str1 = NSString!("test");
            let str2 = send![NSString stringWithUTF8String:c_str!("test")];
            assert!(send![str1 length] == 4);
            assert!(send![str2 length] == 4);
            assert!(send![(str1) isEqual:str2] != 0);
        }
    }

    #[test]
    pub fn test_format() {
        unsafe {
            let NSString = Class!(NSString);
            let str = send![NSString stringWithFormat:NSString!("%@, %@!"), NSString!("Hello"), NSString!("World")];
            assert!(send![(str) isEqual:NSString!("Hello, World!")] == 1)
        }
    }

    #[test]
    pub fn test_nslog() {
        unsafe {
            let NSString = Class!(NSString);
            let name = objc::NSStringFromClass(NSString);
            assert!(name != objc::nil);
            assert!(send![name length] == 8);
            objc::NSLog(name);

            let format = NSString!("Test Log");
            assert!(format != objc::nil);
            objc::NSLog(format);
        }
    }

    #[test]
    pub fn test_array() {
        let nsnull = unsafe { send![(Class!(NSNull)) null] };
        unsafe {
            let array = NSArray![];
            assert!(send![array count] == 0);
        }
        unsafe {
            let array = NSArray![nsnull];
            assert!(send![array count] == 1);
            assert!(send![(array) objectAtIndex:0] == nsnull);
        }
        unsafe {
            let array = NSArray![nsnull, nsnull];
            assert!(send![array count] == 2);
            assert!(send![(array) objectAtIndex:0] == nsnull);
        }
        unsafe {
            let nsnull = send![(Class!(NSNull)) null];
            let array = NSArray![nsnull, nsnull, nsnull];
            assert!(send![array count] == 3);
        }
    }

    #[test]
    pub fn test_dictionary() {
        let nsnull = unsafe { send![(Class!(NSNull)) null] };
        unsafe {
            let dict = NSDictionary![];
            assert!(send![dict count] == 0);
        }
        unsafe {
            let key = NSString!("key");
            let dict = NSDictionary![key => nsnull];
            assert!(send![dict count] == 1);
            assert!(send![(dict) objectForKey:key] == nsnull);
        }
        unsafe {
            let key = NSString!("key");
            let dict = NSDictionary![key => nsnull, NSString!("key2") => nsnull];
            assert!(send![dict count] == 2);
            assert!(send![(dict) objectForKey:key] == nsnull);
            assert!(send![(dict) objectForKey:NSString!("key2")] == nsnull);
        }
    }

    #[test]
    pub fn test_cocoa() {
        unsafe {
            let NSBundle = Class!(NSBundle);
            let bundle = send![NSBundle mainBundle];
            let bundleURL = send![bundle bundleURL];
            let resourceURL = send![bundle resourceURL];
            let executableURL = send![bundle executableURL];
            objc::NSLog(NSString!("%@\n%@\n%@\n"), bundleURL, resourceURL, executableURL);
        }
    }
}

