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
use std::time::Duration;
use pbr::MultiBar;
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
struct pbr_progress_bar_t {
    progress_bar: *mut c_void,
    handle: pbr_handle_t
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum pbr_duration_t {
    PBR_DURATION_MICROS,
    PBR_DURATION_MILLIS,
    PBR_DURATION_NANOS,
    PBR_DURATION_SECS,
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum pbr_units_t {
    PBR_UNITS_DEFAULT,
    PBR_UNITS_BYTES,
}

fn c_units_to_rust_units(units: pbr_units_t) -> Units {
    match units {
	pbr_units_t::PBR_UNITS_DEFAULT => Units::Default,
	pbr_units_t::PBR_UNITS_BYTES => Units::Bytes
    }
}

fn c_duration_to_rust_duration(duration: pbr_duration_t, w: u64) -> Duration {
    match duration {
	pbr_duration_t::PBR_DURATION_MICROS => Duration::from_micros(w),
	pbr_duration_t::PBR_DURATION_MILLIS => Duration::from_millis(w),
	pbr_duration_t::PBR_DURATION_NANOS => Duration::from_nanos(w),
	pbr_duration_t::PBR_DURATION_SECS => Duration::from_secs(w),
    }
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
	progress_bar: Box::into_raw(Box::new(pb)) as *mut c_void,
	handle: pbr_handle_t::PBR_HANDLE_STDOUT
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

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_new(total: u64) -> pbr_progress_bar_t {
    let progress_bar = ProgressBar::new(total);
    pbr_progress_bar_t {
	progress_bar: Box::into_raw(Box::new(progress_bar)) as *mut c_void,
	handle: pbr_handle_t::PBR_HANDLE_STDOUT
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_on(handle: pbr_handle_t, total: u64) -> pbr_progress_bar_t {
    if handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let progress_bar = ProgressBar::on(stdout(), total);
	pbr_progress_bar_t {
	    progress_bar: Box::into_raw(Box::new(progress_bar)) as *mut c_void,
	    handle: pbr_handle_t::PBR_HANDLE_STDOUT
	}
    } else {
	let progress_bar = ProgressBar::on(stderr(), total);
	pbr_progress_bar_t {
	    progress_bar: Box::into_raw(Box::new(progress_bar)) as *mut c_void,
	    handle: pbr_handle_t::PBR_HANDLE_STDERR
	}
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_set_units(progress_bar: *mut pbr_progress_bar_t, units: pbr_units_t) {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.set_units(c_units_to_rust_units(units));
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.set_units(c_units_to_rust_units(units));
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_format(progress_bar: *mut pbr_progress_bar_t, fmt: *const c_char) {
    let cstr = match CStr::from_ptr(fmt).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.format(cstr);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.format(cstr);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_message(progress_bar: *mut pbr_progress_bar_t, message: *const c_char) {
    let cstr = match CStr::from_ptr(message).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.message(cstr);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.message(cstr);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_tick_format(progress_bar: *mut pbr_progress_bar_t, tick_fmt: *const c_char) {
    let cstr = match CStr::from_ptr(tick_fmt).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.tick_format(cstr);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.tick_format(cstr);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_set_width(progress_bar: *mut pbr_progress_bar_t, w: usize) {
    let rw = if w == 0 { None } else { Some(w) };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.set_width(rw);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.set_width(rw);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_set_max_refresh_rate(progress_bar: *mut pbr_progress_bar_t, duration: pbr_duration_t, w: u64) {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.set_max_refresh_rate(if w == 0 { None } else { Some(c_duration_to_rust_duration(duration, w)) });
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.set_max_refresh_rate(if w == 0 { None } else { Some(c_duration_to_rust_duration(duration, w)) });
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_tick(progress_bar: *mut pbr_progress_bar_t) {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.tick();
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.tick();
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_add(progress_bar: *mut pbr_progress_bar_t, i: u64) -> u64 {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	return mb.add(i);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	return mb.add(i);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_set(progress_bar: *mut pbr_progress_bar_t, i: u64) -> u64 {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	return mb.set(i);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	return mb.set(i);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_inc(progress_bar: *mut pbr_progress_bar_t) -> u64 {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	return mb.inc();
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	return mb.inc();
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_reset_start_time(progress_bar: *mut pbr_progress_bar_t) {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.reset_start_time();
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.reset_start_time();
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_finish(progress_bar: *mut pbr_progress_bar_t) {
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.finish();
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.finish();
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_finish_print(progress_bar: *mut pbr_progress_bar_t, s: *const c_char) {
    let cstr = match CStr::from_ptr(s).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.finish_print(cstr);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.finish_print(cstr);
    }
}

#[no_mangle]
unsafe extern "C" fn pbr_progress_bar_finish_println(progress_bar: *mut pbr_progress_bar_t, s: *const c_char) {
    let cstr = match CStr::from_ptr(s).to_str() {
	Ok(s) => s,
	Err(_) => return
    };
    if (*progress_bar).handle == pbr_handle_t::PBR_HANDLE_STDOUT {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stdout>);
	mb.finish_println(cstr);
    } else {
	let mb = &mut *((*progress_bar).progress_bar as *mut ProgressBar<Stderr>);
	mb.finish_println(cstr);
    }
}

mod test {

    use std::thread;
    use crate::*;
    
    #[test]
    fn test() {
	unsafe {
	    let count = 100;
	    let mut pb = pbr_progress_bar_new(count);
	    pbr_progress_bar_format(&mut pb as *mut pbr_progress_bar_t,"╢▌▌░╟\0".as_ptr() as *const c_char);
	    for _ in 0..count {
		pbr_progress_bar_inc(&mut pb as *mut pbr_progress_bar_t);
		#[allow(deprecated)]
		thread::sleep_ms(200);
	    }
	}
    }
}
