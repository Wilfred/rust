//@ compile-flags: -Zunstable-options --generate-link-to-definition
//@ check-pass

#![crate_type = "lib"]

pub struct Socket;

impl Socket {
    fn local_method(&self) {}

    pub fn documented_method() {
        let socket = Socket;
        socket.local_method();

        crate::sys::socketpair();
    }
}

mod sys {}
