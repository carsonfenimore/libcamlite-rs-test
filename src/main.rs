use rslibcamlitelib::LibCamClient;
use rslibcamlitelib::StreamParams;
use rslibcamlitelib::StreamFormat;
use rslibcamlitelib::ExternalCallback;

mod timereporter;
use timereporter::RateReporter;

struct MyCallback {
    h264Reporter: RateReporter,
    lowReporter: RateReporter,
}

impl MyCallback {
    fn new() -> Self{
        MyCallback{
            h264Reporter: RateReporter::new("h264"),
            lowReporter: RateReporter::new("low")
        }
    }
}

#[allow(non_snake_case, deprecated)]
impl ExternalCallback for MyCallback {
    unsafe fn callbackH264(&mut self, _bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool ){
        self.h264Reporter.tick();
    }
    unsafe fn callbackLowres(&mut self, _bytes: *mut u8, count: usize){
        self.lowReporter.tick();
    }
}

#[allow(non_snake_case)]
fn main() {
    println!("Creating client\n");
    let libcam = LibCamClient::new();

    // Setup streams
    println!("Setting up low res stream\n");
    libcam.client.setupLowres(StreamParams{ width: 300, height: 300, format: StreamFormat::STREAM_FORMAT_RGB, framerate: 30});

    println!("Setting up high res stream\n");
    let h264Params = StreamParams{ width: 1920, height: 1080, format:  StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    println!("Calling into setupH264\n");
    libcam.client.setupH264(h264Params, 5, "main".to_owned(), "2mbps".to_owned());

    let mycb = Box::new(MyCallback::new());
    libcam.setCallbacks(mycb);
    println!("Running...\n");
    libcam.run();
}
