# libcamlite-rs-test
Demo app for exercising libcamlite-rs.

## Using this library
You can get parallel RGB/h264 streams in just a few lines:

    let libcam = LibCamClient::new();

    let lowres = StreamParams{ width: 300, height: 300, format: StreamFormat::STREAM_FORMAT_RGB, framerate: 30};
    libcam.client.setupLowres(&lowres);

    let h264Params = StreamParams{ width: 1920, height: 1080, format:  StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    libcam.client.setupH264(&h264Params, 5, &"main".to_owned(), &"2mbps".to_owned());

    let mycb = Box::new(MyCallback::new(lowres));
    libcam.setCallbacks(mycb);
    libcam.run();

## Building
To build this app ensure you have libcamlite built:

  git clone https://github.com/carsonfenimore/libcamlite
  # follow instructions to build libcamlite
  

Then build this app:

    git clone https://github.com/carsonfenimore/libcamlite-rs-test
    cd libcamlite-rs-test
    LD_LIBRARY_PATH=../libcamlite/build cargo run


You should then see output like the following:

    low: 31 calls in past 1.0325267 seconds; 30.023436 calls/sec
    h264: 31 calls in past 1.0341291 seconds; 29.976913 calls/sec
    Wrote output.ppm
    low: 31 calls in past 1.0325396 seconds; 30.023062 calls/sec
    h264: 30 calls in past 1.0032188 seconds; 29.903748 calls/sec

All in about 70MB of RAM and a load average of 0.5 on a raspberry pi zero 2w, running bullseye aarch64! View the output.h264 file in VLC.  View the PPM using any image viewer.
