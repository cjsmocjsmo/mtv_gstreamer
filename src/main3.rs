use rdev::display_size;
use std::ffi::OsStr;
use std::process::Command;

fn main() {
    let path = "/home/charliepi/testvid.mp4";
    let ur = format!("uri=file://{}", path);
    let uri: &OsStr = ur.as_ref();

    let (w, h) = display_size().unwrap();
    let width = w - 175;
    let height = h - 175;

    let v1 = format!(
        "videoscale ! video/x-raw,width={},height={} ! xvimagesink",
        width, height
    );
    let v2 = format!("video-sink=\"{}\"", v1);
    let videosink: &OsStr = v2.as_ref();

    Command::new("gst-launch-1.0")
        .arg("playbin")
        .arg(uri)
        .arg(videosink)
        .arg("audio-sink=pulsesink")
        .spawn()
        .expect("Failed to start video playback");
}
