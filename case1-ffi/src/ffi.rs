use libc;

extern "C" {
    pub fn good_job();
    fn nice() -> libc::c_int;
}

pub fn cool() -> bool {
    unsafe { nice() != 0 }
}
