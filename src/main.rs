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

# Simulated change for commit 83 on 2023-04-25 15:04:47

# Simulated change for commit 84 on 2023-05-01 13:56:35

# Simulated change for commit 85 on 2023-05-02 16:39:35

# Simulated change for commit 86 on 2023-05-03 17:54:55

# Simulated change for commit 89 on 2023-05-05 16:50:27

# Simulated change for commit 91 on 2023-05-08 10:26:36

# Simulated change for commit 93 on 2023-05-11 13:15:52

# Simulated change for commit 95 on 2023-05-15 17:00:31

# Simulated change for commit 97 on 2023-05-19 16:57:32

# Simulated change for commit 98 on 2023-05-19 15:38:38

# Simulated change for commit 102 on 2023-05-24 10:33:12

# Simulated change for commit 103 on 2023-05-24 17:08:41

# Simulated change for commit 106 on 2023-05-26 15:04:21

# Simulated change for commit 107 on 2023-05-26 11:08:37

# Simulated change for commit 108 on 2023-05-29 10:25:48

# Simulated change for commit 111 on 2023-05-31 14:08:48

# Simulated change for commit 112 on 2023-06-02 12:53:56

# Simulated change for commit 113 on 2023-06-02 17:38:08

# Simulated change for commit 114 on 2023-06-05 14:41:47

# Simulated change for commit 115 on 2023-06-05 09:27:31

# Simulated change for commit 117 on 2023-06-12 10:38:07

# Simulated change for commit 121 on 2023-06-19 13:49:54

# Simulated change for commit 123 on 2023-06-20 09:20:25

# Simulated change for commit 124 on 2023-06-21 13:38:05

# Simulated change for commit 126 on 2023-06-26 11:23:11

# Simulated change for commit 129 on 2023-06-28 10:18:03

# Simulated change for commit 130 on 2023-06-29 14:39:45

# Simulated change for commit 134 on 2023-06-30 17:26:09

# Simulated change for commit 135 on 2023-07-03 15:17:04

# Simulated change for commit 138 on 2023-07-06 17:26:57

# Simulated change for commit 139 on 2023-07-10 09:02:30

# Simulated change for commit 140 on 2023-07-10 11:06:12

# Simulated change for commit 142 on 2023-07-18 13:10:50

# Simulated change for commit 143 on 2023-07-19 09:00:07

# Simulated change for commit 146 on 2023-07-28 17:39:40

# Simulated change for commit 148 on 2023-07-31 11:05:30

# Simulated change for commit 151 on 2023-08-01 16:01:24

# Simulated change for commit 152 on 2023-08-02 14:43:52

# Simulated change for commit 154 on 2023-08-03 14:39:07

# Simulated change for commit 155 on 2023-08-03 12:49:52

# Simulated change for commit 156 on 2023-08-07 16:51:08

# Simulated change for commit 159 on 2023-08-08 14:12:42

# Simulated change for commit 164 on 2023-08-11 13:02:45

# Simulated change for commit 165 on 2023-08-11 14:07:59

# Simulated change for commit 167 on 2023-08-17 11:10:34

# Simulated change for commit 168 on 2023-08-18 16:51:13

# Simulated change for commit 169 on 2023-08-21 13:05:38

# Simulated change for commit 170 on 2023-08-22 13:00:31

# Simulated change for commit 172 on 2023-08-24 09:49:07

# Simulated change for commit 175 on 2023-09-04 13:36:33

# Simulated change for commit 177 on 2023-09-06 15:54:39

# Simulated change for commit 179 on 2023-09-08 16:41:51

# Simulated change for commit 184 on 2023-09-20 12:05:00

# Simulated change for commit 186 on 2023-09-20 16:17:54

# Simulated change for commit 188 on 2023-09-22 15:45:52

# Simulated change for commit 190 on 2023-09-25 10:55:46

# Simulated change for commit 192 on 2023-09-28 13:54:15

# Simulated change for commit 199 on 2023-10-13 10:33:33
