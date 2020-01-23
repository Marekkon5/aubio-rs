/* automatically generated by rust-bindgen */

pub const HAVE_AUBIO_DOUBLE: u32 = 0;
pub const AUBIO_SMPL_FMT: &'static [u8; 3usize] = b"%f\0";
pub const AUBIO_LSMP_FMT: &'static [u8; 4usize] = b"%lf\0";
pub type smpl_t = f32;
pub type lsmp_t = f64;
#[doc = " unsigned integer"]
pub type uint_t = ::std::os::raw::c_uint;
#[doc = " signed integer"]
pub type sint_t = ::std::os::raw::c_int;
#[doc = " character"]
pub type char_t = ::std::os::raw::c_char;
#[doc = " Buffer for real data"]
#[doc = ""]
#[doc = "Vector of real-valued data"]
#[doc = ""]
#[doc = "::fvec_t is is the structure used to store vector of real-valued data, ::smpl_t ."]
#[doc = ""]
#[doc = "\\code"]
#[doc = ""]
#[doc = "uint_t buffer_size = 1024;"]
#[doc = ""]
#[doc = "fvec_t * input = new_fvec (buffer_size);"]
#[doc = ""]
#[doc = "input->data[23] = 2.;"]
#[doc = ""]
#[doc = "mean = fvec_mean(a_vector);"]
#[doc = ""]
#[doc = "del_fvec(a_vector);"]
#[doc = ""]
#[doc = "\\endcode"]
#[doc = ""]
#[doc = "See `examples/` and `tests/src` directories for more examples."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fvec_t {
    #[doc = "< length of buffer"]
    pub length: uint_t,
    #[doc = "< data vector of length ::fvec_t.length"]
    pub data: *mut smpl_t,
}
#[test]
fn bindgen_test_layout_fvec_t() {
    assert_eq!(
        ::std::mem::size_of::<fvec_t>(),
        8usize,
        concat!("Size of: ", stringify!(fvec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<fvec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(fvec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fvec_t>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fvec_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fvec_t>())).data as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fvec_t),
            "::",
            stringify!(data)
        )
    );
}
#[doc = " Vector of real-valued phase and spectrum data"]
#[doc = ""]
#[doc = "\\code"]
#[doc = ""]
#[doc = "uint_t buffer_size = 1024;"]
#[doc = ""]
#[doc = "cvec_t * input = new_cvec (buffer_size);"]
#[doc = ""]
#[doc = "input->norm[23] = 2.;"]
#[doc = "input->phas[23] = M_PI;"]
#[doc = ""]
#[doc = "mean = cvec_mean(input);"]
#[doc = ""]
#[doc = "del_cvec (input);"]
#[doc = ""]
#[doc = "\\endcode"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cvec_t {
    #[doc = "< length of buffer = (requested length)/2 + 1"]
    pub length: uint_t,
    #[doc = "< norm array of size ::cvec_t.length"]
    pub norm: *mut smpl_t,
    #[doc = "< phase array of size ::cvec_t.length"]
    pub phas: *mut smpl_t,
}
#[test]
fn bindgen_test_layout_cvec_t() {
    assert_eq!(
        ::std::mem::size_of::<cvec_t>(),
        12usize,
        concat!("Size of: ", stringify!(cvec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<cvec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(cvec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cvec_t>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cvec_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cvec_t>())).norm as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(cvec_t),
            "::",
            stringify!(norm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cvec_t>())).phas as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cvec_t),
            "::",
            stringify!(phas)
        )
    );
}
#[doc = " Buffer for real data in double precision"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lvec_t {
    #[doc = "< length of buffer"]
    pub length: uint_t,
    #[doc = "< data array of size [length]"]
    pub data: *mut lsmp_t,
}
#[test]
fn bindgen_test_layout_lvec_t() {
    assert_eq!(
        ::std::mem::size_of::<lvec_t>(),
        8usize,
        concat!("Size of: ", stringify!(lvec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lvec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(lvec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lvec_t>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lvec_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lvec_t>())).data as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(lvec_t),
            "::",
            stringify!(data)
        )
    );
}
#[doc = " Buffer for real data"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmat_t {
    #[doc = "< length of matrix"]
    pub length: uint_t,
    #[doc = "< height of matrix"]
    pub height: uint_t,
    #[doc = "< data array of size [length] * [height]"]
    pub data: *mut *mut smpl_t,
}
#[test]
fn bindgen_test_layout_fmat_t() {
    assert_eq!(
        ::std::mem::size_of::<fmat_t>(),
        12usize,
        concat!("Size of: ", stringify!(fmat_t))
    );
    assert_eq!(
        ::std::mem::align_of::<fmat_t>(),
        4usize,
        concat!("Alignment of ", stringify!(fmat_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fmat_t>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fmat_t),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fmat_t>())).height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fmat_t),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fmat_t>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fmat_t),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_resampler_t {
    _unused: [u8; 0],
}
#[doc = " resampler object"]
pub type aubio_resampler_t = _aubio_resampler_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_filter_t {
    _unused: [u8; 0],
}
#[doc = " Digital filter"]
pub type aubio_filter_t = _aubio_filter_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_fft_t {
    _unused: [u8; 0],
}
#[doc = " FFT object"]
#[doc = ""]
#[doc = "This object computes forward and backward FFTs."]
pub type aubio_fft_t = _aubio_fft_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_dct_t {
    _unused: [u8; 0],
}
#[doc = " DCT object"]
#[doc = ""]
#[doc = "This object computes forward and backward DCT type 2 with orthonormal"]
#[doc = "scaling."]
pub type aubio_dct_t = _aubio_dct_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_pvoc_t {
    _unused: [u8; 0],
}
#[doc = " phasevocoder object"]
pub type aubio_pvoc_t = _aubio_pvoc_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_filterbank_t {
    _unused: [u8; 0],
}
#[doc = " filterbank object"]
#[doc = ""]
#[doc = "This object stores a matrix of spectral filter coefficients."]
#[doc = ""]
pub type aubio_filterbank_t = _aubio_filterbank_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_mfcc_t {
    _unused: [u8; 0],
}
#[doc = " mfcc object"]
pub type aubio_mfcc_t = _aubio_mfcc_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_specdesc_t {
    _unused: [u8; 0],
}
#[doc = " spectral description structure"]
pub type aubio_specdesc_t = _aubio_specdesc_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_spectral_whitening_t {
    _unused: [u8; 0],
}
#[doc = " spectral whitening structure"]
pub type aubio_spectral_whitening_t = _aubio_spectral_whitening_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_tss_t {
    _unused: [u8; 0],
}
#[doc = " Transient / Steady-state Separation object"]
pub type aubio_tss_t = _aubio_tss_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_pitch_t {
    _unused: [u8; 0],
}
#[doc = " pitch detection object"]
pub type aubio_pitch_t = _aubio_pitch_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_onset_t {
    _unused: [u8; 0],
}
#[doc = " onset detection object"]
pub type aubio_onset_t = _aubio_onset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_tempo_t {
    _unused: [u8; 0],
}
#[doc = " tempo detection structure"]
pub type aubio_tempo_t = _aubio_tempo_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_notes_t {
    _unused: [u8; 0],
}
#[doc = " notes detection object"]
pub type aubio_notes_t = _aubio_notes_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_source_t {
    _unused: [u8; 0],
}
#[doc = " media source object"]
pub type aubio_source_t = _aubio_source_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_sink_t {
    _unused: [u8; 0],
}
#[doc = " media sink object"]
pub type aubio_sink_t = _aubio_sink_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_sampler_t {
    _unused: [u8; 0],
}
#[doc = " sampler object"]
pub type aubio_sampler_t = _aubio_sampler_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_wavetable_t {
    _unused: [u8; 0],
}
#[doc = " wavetable object"]
pub type aubio_wavetable_t = _aubio_wavetable_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _aubio_parameter_t {
    _unused: [u8; 0],
}
#[doc = " parameter object"]
pub type aubio_parameter_t = _aubio_parameter_t;
#[doc = "< critical errors"]
pub const aubio_log_level_AUBIO_LOG_ERR: aubio_log_level = 0;
#[doc = "< infos"]
pub const aubio_log_level_AUBIO_LOG_INF: aubio_log_level = 1;
#[doc = "< general messages"]
pub const aubio_log_level_AUBIO_LOG_MSG: aubio_log_level = 2;
#[doc = "< debug messages"]
pub const aubio_log_level_AUBIO_LOG_DBG: aubio_log_level = 3;
#[doc = "< warnings"]
pub const aubio_log_level_AUBIO_LOG_WRN: aubio_log_level = 4;
#[doc = "< number of valid levels"]
pub const aubio_log_level_AUBIO_LOG_LAST_LEVEL: aubio_log_level = 5;
#[doc = " list of logging levels"]
pub type aubio_log_level = u32;
#[doc = " Logging function prototype, to be passed to ::aubio_log_set_function"]
#[doc = ""]
#[doc = "\\param level log level"]
#[doc = "\\param message text to log"]
#[doc = "\\param data optional closure used by the callback"]
#[doc = ""]
#[doc = "See @ref utils/test-log.c for an example of logging function."]
#[doc = ""]
pub type aubio_log_function_t = ::std::option::Option<
    unsafe extern "C" fn(level: sint_t, message: *const char_t, data: *mut ::std::os::raw::c_void),
>;
