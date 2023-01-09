use opencv::{
    core::{Mat, MatTraitConst, Scalar, Size, CV_8UC3},
    highgui::{imshow, wait_key},
    imgcodecs::{imdecode, imread, IMREAD_COLOR},
    imgproc::{cvt_color, rectangle, COLOR_BGR2GRAY},
    objdetect::CascadeClassifier,
    prelude::*,
    videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY},
};
use std::error::Error;

const WINDOW: &str = "Real-time Computer Vision Pipeline";
const CASCADE_PATH: &str = "./haarcascade_frontalface_alt.xml"; // Ensure this file is present

fn run() -> Result<(), Box<dyn Error>> {
    // 1. Initialize video capture
    let mut camera = VideoCapture::new(0, CAP_ANY)?; // 0 is the default camera
    if !VideoCaptureTrait::is_opened(&camera) {
        return Err("Unable to open default camera!".into());
    }

    // 2. Load face cascade classifier
    let mut face_classifier = CascadeClassifier::new(CASCADE_PATH)?;
    if CascadeClassifier::empty(&face_classifier) {
        return Err(format!("Could not load face cascade classifier from {}", CASCADE_PATH).into());
    }

    println!("Starting real-time computer vision pipeline. Press ESC to exit.");

    loop {
        let mut frame = Mat::default();
        camera.read(&mut frame)?;

        if frame.empty() {
            println!("End of stream");
            break;
        }

        // 3. Convert frame to grayscale for face detection
        let mut gray_frame = Mat::default();
        cvt_color(&frame, &mut gray_frame, COLOR_BGR2GRAY, 0)?;

        // 4. Detect faces
        let mut faces = opencv::core::VectorOfRect::new();
        face_classifier.detect_multi_scale(
            &gray_frame,
            &mut faces,
            1.1, // scaleFactor
            2,   // minNeighbors
            0,   // flags
            Size::new(30, 30), // minSize
            Size::new(0, 0),   // maxSize (0,0 means no limit)
        )?;

        // 5. Draw rectangles around detected faces
        for face in faces.iter() {
            rectangle(
                &mut frame,
                face,
                Scalar::new(0.0, 255.0, 0.0, 0.0), // Green color
                2, // thickness
                opencv::imgproc::LINE_8,
                0,
            )?;
        }

        // 6. Display the result
        imshow(WINDOW, &frame)?;

        let key = wait_key(10)?; // Wait for 10ms
        if key == 27 { // ESC key
            println!("ESC pressed, exiting.");
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

# Simulated change for commit 3 on 2023-01-09 17:15:42
