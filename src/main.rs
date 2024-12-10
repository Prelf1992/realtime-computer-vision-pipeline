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

# Simulated change for commit 200 on 2023-10-19 11:43:30

# Simulated change for commit 201 on 2023-10-19 10:56:51

# Simulated change for commit 202 on 2023-10-20 12:34:48

# Simulated change for commit 203 on 2023-10-20 14:11:46

# Simulated change for commit 204 on 2023-10-20 17:44:21

# Simulated change for commit 206 on 2023-10-24 17:37:34

# Simulated change for commit 207 on 2023-10-26 12:48:55

# Simulated change for commit 210 on 2023-10-31 14:16:42

# Simulated change for commit 211 on 2023-11-01 14:19:21

# Simulated change for commit 212 on 2023-11-02 09:15:26

# Simulated change for commit 213 on 2023-11-03 10:24:00

# Simulated change for commit 214 on 2023-11-07 13:46:47

# Simulated change for commit 215 on 2023-11-08 13:47:09

# Simulated change for commit 216 on 2023-11-09 15:47:20

# Simulated change for commit 217 on 2023-11-10 15:42:03

# Simulated change for commit 219 on 2023-11-14 14:24:22

# Simulated change for commit 222 on 2023-11-21 14:28:37

# Simulated change for commit 224 on 2023-11-22 12:01:08

# Simulated change for commit 229 on 2023-11-29 11:36:09

# Simulated change for commit 230 on 2023-11-30 11:24:21

# Simulated change for commit 233 on 2023-12-04 15:00:43

# Simulated change for commit 235 on 2023-12-05 09:37:13

# Simulated change for commit 236 on 2023-12-05 17:00:13

# Simulated change for commit 246 on 2023-12-25 15:22:50

# Simulated change for commit 248 on 2023-12-26 15:14:43

# Simulated change for commit 250 on 2023-12-28 15:42:28

# Simulated change for commit 2 on 2024-01-03 09:59:23

# Simulated change for commit 3 on 2024-01-04 12:17:12

# Simulated change for commit 4 on 2024-01-05 13:15:43

# Simulated change for commit 5 on 2024-01-08 09:10:08

# Simulated change for commit 6 on 2024-01-08 10:35:00

# Simulated change for commit 8 on 2024-01-12 16:44:00

# Simulated change for commit 9 on 2024-01-15 10:23:39

# Simulated change for commit 11 on 2024-01-16 14:18:05

# Simulated change for commit 13 on 2024-01-17 14:20:37

# Simulated change for commit 14 on 2024-01-17 15:51:50

# Simulated change for commit 15 on 2024-01-18 17:36:40

# Simulated change for commit 17 on 2024-01-22 13:32:52

# Simulated change for commit 19 on 2024-01-24 09:18:52

# Simulated change for commit 20 on 2024-01-26 10:22:53

# Simulated change for commit 24 on 2024-01-31 12:49:04

# Simulated change for commit 25 on 2024-02-01 16:45:55

# Simulated change for commit 26 on 2024-02-01 13:08:39

# Simulated change for commit 27 on 2024-02-02 14:24:34

# Simulated change for commit 28 on 2024-02-05 12:30:27

# Simulated change for commit 29 on 2024-02-05 13:52:26

# Simulated change for commit 31 on 2024-02-08 12:03:52

# Simulated change for commit 36 on 2024-02-13 17:32:11

# Simulated change for commit 39 on 2024-02-20 17:41:04

# Simulated change for commit 40 on 2024-02-22 17:47:11

# Simulated change for commit 42 on 2024-02-23 10:42:06

# Simulated change for commit 46 on 2024-03-01 13:26:14

# Simulated change for commit 47 on 2024-03-01 11:23:38

# Simulated change for commit 48 on 2024-03-05 16:12:03

# Simulated change for commit 49 on 2024-03-06 17:40:27

# Simulated change for commit 50 on 2024-03-06 13:38:37

# Simulated change for commit 53 on 2024-03-08 11:04:32

# Simulated change for commit 55 on 2024-03-12 16:41:51

# Simulated change for commit 56 on 2024-03-18 11:34:08

# Simulated change for commit 57 on 2024-03-20 11:56:44

# Simulated change for commit 59 on 2024-03-25 11:22:13

# Simulated change for commit 60 on 2024-03-25 10:37:10

# Simulated change for commit 61 on 2024-03-26 16:34:49

# Simulated change for commit 62 on 2024-03-26 13:30:35

# Simulated change for commit 65 on 2024-03-28 10:45:49

# Simulated change for commit 67 on 2024-04-03 11:50:27

# Simulated change for commit 69 on 2024-04-04 13:47:31

# Simulated change for commit 73 on 2024-04-11 10:06:43

# Simulated change for commit 75 on 2024-04-12 17:12:27

# Simulated change for commit 76 on 2024-04-15 14:22:11

# Simulated change for commit 77 on 2024-04-18 14:29:47

# Simulated change for commit 78 on 2024-04-19 11:20:05

# Simulated change for commit 81 on 2024-04-25 11:27:51

# Simulated change for commit 83 on 2024-04-26 11:16:55

# Simulated change for commit 84 on 2024-04-26 09:17:29

# Simulated change for commit 86 on 2024-04-29 11:50:25

# Simulated change for commit 88 on 2024-05-01 14:38:17

# Simulated change for commit 89 on 2024-05-01 11:36:58

# Simulated change for commit 90 on 2024-05-02 16:00:35

# Simulated change for commit 91 on 2024-05-03 11:39:43

# Simulated change for commit 92 on 2024-05-06 16:16:33

# Simulated change for commit 96 on 2024-05-10 15:28:28

# Simulated change for commit 104 on 2024-05-21 16:02:01

# Simulated change for commit 105 on 2024-05-22 12:54:08

# Simulated change for commit 106 on 2024-05-23 11:30:27

# Simulated change for commit 107 on 2024-05-23 16:28:48

# Simulated change for commit 109 on 2024-05-24 13:23:26

# Simulated change for commit 111 on 2024-05-28 17:49:37

# Simulated change for commit 113 on 2024-05-29 17:10:10

# Simulated change for commit 115 on 2024-05-30 12:34:50

# Simulated change for commit 116 on 2024-05-31 09:26:58

# Simulated change for commit 117 on 2024-05-31 13:13:55

# Simulated change for commit 120 on 2024-06-05 16:08:29

# Simulated change for commit 121 on 2024-06-10 10:41:31

# Simulated change for commit 123 on 2024-06-12 15:53:34

# Simulated change for commit 126 on 2024-06-14 13:21:54

# Simulated change for commit 127 on 2024-06-17 17:48:58

# Simulated change for commit 128 on 2024-06-17 11:47:29

# Simulated change for commit 130 on 2024-06-24 10:14:04

# Simulated change for commit 131 on 2024-06-24 16:30:29

# Simulated change for commit 132 on 2024-06-27 11:44:00

# Simulated change for commit 135 on 2024-07-08 13:17:43

# Simulated change for commit 137 on 2024-07-16 16:07:36

# Simulated change for commit 138 on 2024-07-22 10:34:35

# Simulated change for commit 142 on 2024-07-29 12:32:49

# Simulated change for commit 147 on 2024-07-31 15:02:40

# Simulated change for commit 149 on 2024-08-02 14:29:24

# Simulated change for commit 150 on 2024-08-05 11:52:57

# Simulated change for commit 153 on 2024-08-07 10:01:19

# Simulated change for commit 154 on 2024-08-08 09:57:13

# Simulated change for commit 155 on 2024-08-09 16:28:00

# Simulated change for commit 156 on 2024-08-09 17:56:41

# Simulated change for commit 157 on 2024-08-13 09:42:26

# Simulated change for commit 161 on 2024-08-27 16:20:48

# Simulated change for commit 162 on 2024-08-28 14:55:16

# Simulated change for commit 163 on 2024-08-28 17:28:15

# Simulated change for commit 164 on 2024-08-29 12:17:50

# Simulated change for commit 165 on 2024-08-29 17:01:00

# Simulated change for commit 166 on 2024-09-04 14:26:28

# Simulated change for commit 170 on 2024-09-06 15:20:30

# Simulated change for commit 171 on 2024-09-06 12:00:31

# Simulated change for commit 172 on 2024-09-09 13:22:57

# Simulated change for commit 173 on 2024-09-09 11:17:52

# Simulated change for commit 175 on 2024-09-12 12:44:03

# Simulated change for commit 176 on 2024-09-16 16:43:20

# Simulated change for commit 178 on 2024-09-17 13:12:41

# Simulated change for commit 180 on 2024-09-19 11:35:33

# Simulated change for commit 182 on 2024-09-25 12:32:07

# Simulated change for commit 190 on 2024-10-04 15:18:13

# Simulated change for commit 196 on 2024-10-16 09:03:01

# Simulated change for commit 198 on 2024-10-24 11:30:40

# Simulated change for commit 199 on 2024-10-28 10:26:50

# Simulated change for commit 200 on 2024-10-28 11:46:02

# Simulated change for commit 201 on 2024-10-28 09:36:21

# Simulated change for commit 202 on 2024-10-29 14:36:41

# Simulated change for commit 203 on 2024-10-30 13:20:43

# Simulated change for commit 205 on 2024-10-30 17:26:20

# Simulated change for commit 208 on 2024-11-01 09:07:29

# Simulated change for commit 209 on 2024-11-04 10:45:04

# Simulated change for commit 211 on 2024-11-05 09:58:16

# Simulated change for commit 215 on 2024-11-08 12:48:44

# Simulated change for commit 216 on 2024-11-08 15:49:43

# Simulated change for commit 217 on 2024-11-12 10:58:34

# Simulated change for commit 218 on 2024-11-13 17:11:06

# Simulated change for commit 220 on 2024-11-19 11:20:11

# Simulated change for commit 221 on 2024-11-20 17:26:14

# Simulated change for commit 222 on 2024-11-21 09:49:16

# Simulated change for commit 223 on 2024-11-22 11:06:18

# Simulated change for commit 225 on 2024-11-22 11:58:24

# Simulated change for commit 226 on 2024-11-25 15:33:32

# Simulated change for commit 227 on 2024-11-27 11:50:11

# Simulated change for commit 228 on 2024-11-28 11:45:53

# Simulated change for commit 230 on 2024-11-29 16:15:51

# Simulated change for commit 236 on 2024-12-09 14:31:46

# Simulated change for commit 237 on 2024-12-10 12:05:24

# Simulated change for commit 238 on 2024-12-10 17:09:30
