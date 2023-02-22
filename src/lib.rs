/*
* Copyright (c) 2023 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::io::{Stdout, Stderr, stdout, stderr};
use pbr::MultiBar;
use pbr::PbIter;
use pbr::Pipe;
use pbr::ProgressBar;
use pbr::Units;

#[repr(C)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
enum pbr_handle_t {
    PBR_HANDLE_STDERR,
    PBR_HANDLE_STDOUT
}

#[repr(C)]
struct pbr_multi_bar_t {
    multi_bar: *mut c_void,
    handle: pbr_handle_t
}

#[repr(C)]
struct pbr_pbiter_t {
    pbiter: *mut c_void
}

#[repr(C)]
struct pbr_pipe_t {
    pipe: *mut c_void
}

#[repr(C)]
struct pbr_progress_bar_t {
    progress_bar: *mut c_void
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum pbr_units_t {
    PBR_UNITS_DEFAULT,
    PBR_UNITS_BYTES,
}

#[no_mangle]
unsafe extern "C" fn pbr_multi_bar_new() -> pbr_multi_bar_t {
    let multi_bar = MultiBar::new();
    pbr_multi_bar_t {
	multi_bar: Box::into_raw(Box::new(multi_bar)) as *mut c_void,
	handle: pbr_handle_t::PBR_HANDLE_STDOUT
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_multi_bar_on(handle: pbr_handle_t) -> pbr_multi_bar_t {
    if handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let multi_bar = MultiBar::on(stdout());
	pbr_multi_bar_t {
	    multi_bar: Box::into_raw(Box::new(multi_bar)) as *mut c_void,
	    handle: pbr_handle_t::PBR_HANDLE_STDOUT
	}
    } else {
	let multi_bar = MultiBar::on(stderr());
	pbr_multi_bar_t {
	    multi_bar: Box::into_raw(Box::new(multi_bar)) as *mut c_void,
	    handle: pbr_handle_t::PBR_HANDLE_STDERR
	}
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_multi_bar_println(multi_bar: *const pbr_multi_bar_t, s: *const c_char) {
    let cstr = match CStr::from_ptr(s).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*multi_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stdout>);
	mb.println(cstr);
    } else {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stderr>);
	mb.println(cstr);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_multi_bar_create_bar(multi_bar: *const pbr_multi_bar_t, total: u64) -> pbr_progress_bar_t {
    let pb;
    if (*multi_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stdout>);
	pb = mb.create_bar(total);
    } else {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stderr>);
	pb = mb.create_bar(total);
    }
    pbr_progress_bar_t {
	progress_bar: Box::into_raw(Box::new(pb)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_multi_bar_listen(multi_bar: *const pbr_multi_bar_t) {
    if (*multi_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stdout>);
	mb.listen();
    } else {
	let mb = &*((*multi_bar).multi_bar as *mut MultiBar<Stderr>);
	mb.listen();
    }
}
