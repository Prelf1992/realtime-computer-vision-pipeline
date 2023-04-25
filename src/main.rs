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

# Simulated change for commit 9 on 2023-01-12 16:08:14

# Simulated change for commit 12 on 2023-01-17 16:33:51

# Simulated change for commit 13 on 2023-01-18 11:30:58

# Simulated change for commit 14 on 2023-01-18 10:24:33

# Simulated change for commit 15 on 2023-01-18 10:06:31

# Simulated change for commit 17 on 2023-01-20 10:09:50

# Simulated change for commit 18 on 2023-01-20 09:48:57

# Simulated change for commit 21 on 2023-01-26 17:17:42

# Simulated change for commit 22 on 2023-01-26 12:34:01

# Simulated change for commit 25 on 2023-01-31 14:04:17

# Simulated change for commit 32 on 2023-02-07 09:57:41

# Simulated change for commit 33 on 2023-02-07 09:48:35

# Simulated change for commit 34 on 2023-02-07 11:07:19

# Simulated change for commit 35 on 2023-02-10 14:58:43

# Simulated change for commit 36 on 2023-02-10 12:09:40

# Simulated change for commit 37 on 2023-02-13 09:28:20

# Simulated change for commit 38 on 2023-02-13 16:07:44

# Simulated change for commit 40 on 2023-02-14 09:24:12

# Simulated change for commit 44 on 2023-02-20 15:52:47

# Simulated change for commit 47 on 2023-02-23 13:22:38

# Simulated change for commit 48 on 2023-02-24 11:57:46

# Simulated change for commit 51 on 2023-03-02 12:51:50

# Simulated change for commit 53 on 2023-03-03 14:50:11

# Simulated change for commit 55 on 2023-03-13 10:53:37

# Simulated change for commit 56 on 2023-03-14 11:44:25

# Simulated change for commit 58 on 2023-03-15 16:49:18

# Simulated change for commit 64 on 2023-03-23 10:27:30

# Simulated change for commit 65 on 2023-03-27 09:29:29

# Simulated change for commit 66 on 2023-03-27 11:53:21

# Simulated change for commit 69 on 2023-04-04 17:18:08

# Simulated change for commit 72 on 2023-04-10 16:08:46

# Simulated change for commit 73 on 2023-04-10 13:25:18

# Simulated change for commit 80 on 2023-04-24 10:18:54

# Simulated change for commit 81 on 2023-04-25 11:59:43

# Simulated change for commit 82 on 2023-04-25 17:24:18
