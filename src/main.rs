use rslibcamlitelib::{LibCamClient, StreamParams, StreamFormat, ExternalCallback};

use std::fs::File;
use std::io::Write;

mod timereporter;
use timereporter::RateReporter;
mod ppm;

#[allow(non_snake_case, deprecated)]
struct MyCallback {
    lowresParams: StreamParams,
    h264Reporter: RateReporter,
    lowReporter: RateReporter,
    info: *mut std::ffi::c_void,
    stream: *mut sys::AVStream,
}

#[allow(non_snake_case, deprecated)]
impl MyCallback {
    fn new(lowresParams: StreamParams) -> Self{
        MyCallback{
            lowresParams: lowresParams,
            h264Reporter: RateReporter::new(1.0, "h264"),
            lowReporter: RateReporter::new(1.0, "low"),
            info: unsafe { begin_analysis() },
            stream: std::ptr::null_mut(),
        }
    }
}


use ffmpeg_sys_next as sys;

extern "C" {
    fn analyze(obj: *mut std::ffi::c_void, mem: *mut u8, count: usize) -> *mut sys::AVStream;
    fn begin_analysis() -> *mut std::ffi::c_void;
}

use ffmpeg_next as ffmpeg;

#[allow(non_snake_case, deprecated)]
impl ExternalCallback for MyCallback {
    unsafe fn callbackH264(&mut self, bytes: *mut u8, count: usize, _timestamp_us: i64, _keyframe: bool ){
        if self.stream.is_null(){
            self.stream = analyze(self.info, bytes, count);
            if !self.stream.is_null(){
                let cpar = (*self.stream).codecpar;
                let mut parcopy = ffmpeg::codec::Parameters::new();
                let parcopy_ptr = parcopy.as_mut_ptr();
                let rat = ffmpeg::util::rational::Rational::new( (*self.stream).time_base.num, (*self.stream).time_base.den);
                sys::avcodec_parameters_copy(parcopy_ptr, cpar);
                println!("Got av stream! {}x{} rat {}", (*parcopy_ptr).width, (*parcopy_ptr).height, rat);
            }
        }

        self.h264Reporter.tick();
        let mut f = File::options()
            .write(true)
            .create(true)
            .append(true).open("output.h264")
            .unwrap();
        let slice = unsafe { std::slice::from_raw_parts(bytes, count) };
        f.write_all( slice ).expect("Failed to write/open h264 file");
    }
    unsafe fn callbackLowres(&mut self, bytes: *mut u8, count: usize){
        if self.lowReporter.isTimeToReport() {
            let slice = unsafe { std::slice::from_raw_parts(bytes, count) };
            let outputFilename = "output.ppm";
            ppm::writePPM( slice, self.lowresParams.width, self.lowresParams.height, outputFilename);
            println!("Wrote {}", outputFilename);
        }
        self.lowReporter.tick();
    }
}

#[allow(non_snake_case)]
fn main() {
    println!("Creating client\n");
    let libcam = LibCamClient::new();


    // Setup streams
    println!("Setting up low res stream\n");
    let lowres = StreamParams{ width: 300, height: 300, format: StreamFormat::STREAM_FORMAT_RGB, framerate: 30};
    libcam.client.setupLowres(&lowres);

    println!("Setting up high res stream\n");
    let h264Params = StreamParams{ width: 1920, height: 1080, format:  StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    println!("Calling into setupH264\n");
    libcam.client.setupH264(&h264Params, 5, &"main".to_owned(), &"2mbps".to_owned());

    let mycb = Box::new(MyCallback::new(lowres));
    libcam.setCallbacks(mycb);
    println!("Running...\n");
    libcam.start(false);
}
