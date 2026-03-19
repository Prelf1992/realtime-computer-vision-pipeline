//! Real-time Computer Vision Pipeline
//! 
//! This module contains the main logic for an end-to-end real-time object detection and tracking pipeline.
//! It's optimized for edge deployment and aims for low-latency video stream analysis.

use opencv::{
    core::{Mat, MatTraitConst, Scalar, Size, Vector},
    highgui::{imshow, wait_key},
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{cvt_color, put_text, COLOR_BGR2GRAY, FONT_HERSHEY_SIMPLEX},
    objdetect::CascadeClassifier,
    videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY},
    Result,
};

const WINDOW_NAME: &str = "Real-time CV Pipeline";
const CASCADE_PATH: &str = "./haarcascade_frontalface_alt.xml"; // Placeholder for a pre-trained model

/// Initializes and runs the real-time computer vision pipeline.
///
/// This function sets up the video capture, loads a pre-trained cascade classifier
/// (e.g., for face detection), and processes video frames in a loop.
/// It demonstrates basic frame reading, processing, and display.
fn run_pipeline() -> Result<()> {
    println!("Initializing real-time computer vision pipeline...");

    // 1. Initialize video capture
    let mut camera = VideoCapture::new(CAP_ANY, 0)?; // 0 is the default camera
    if !VideoCaptureTrait::is_opened(&camera) {
        return Err(opencv::Error::new(0, "Could not open camera"));
    }
    println!("Camera opened successfully.");

    // 2. Load a pre-trained cascade classifier (e.g., for face detection)
    // In a real application, this would be a more sophisticated model (YOLO, SSD, etc.)
    // and the model file would be part of the project assets.
    let mut face_detector = CascadeClassifier::new(CASCADE_PATH)?;
    if face_detector.empty()? {
        println!("Warning: Cascade classifier not found at {}. Face detection will be skipped.", CASCADE_PATH);
        println!("Please download \'haarcascade_frontalface_alt.xml\' and place it in the project root.");
    } else {
        println!("Cascade classifier loaded successfully.");
    }

    // 3. Create a window to display the video feed
    opencv::highgui::named_window(WINDOW_NAME, opencv::highgui::WINDOW_AUTOSIZE)?;
    println!("Display window created.");

    println!("Starting frame processing loop. Press \'q\' to quit.");

    loop {
        let mut frame = Mat::default();
        camera.read(&mut frame)?;

        if frame.empty()? {
            println!("End of video stream or camera disconnected.");
            break;
        }

        // Convert frame to grayscale for face detection (if classifier is loaded)
        let mut gray_frame = Mat::default();
        if !face_detector.empty()? {
            cvt_color(&frame, &mut gray_frame, COLOR_BGR2GRAY, 0)?;

            // Detect faces
            let mut faces = Vector::<opencv::core::Rect>::new();
            face_detector.detect_multi_scale(
                &gray_frame,
                &mut faces,
                1.1, // Scale factor
                10,  // Minimum neighbors
                opencv::objdetect::CASCADE_SCALE_IMAGE,
                Size::new(30, 30), // Minimum size
                Size::new(0, 0),   // Maximum size (0,0 means no limit)
            )?;

            // Draw rectangles around detected faces
            for face in faces.iter() {
                opencv::imgproc::rectangle(
                    &mut frame,
                    face,
                    Scalar::new(0.0, 255.0, 0.0, 0.0), // Green color
                    2,                                 // Thickness
                    opencv::imgproc::LINE_8,
                    0,
                )?;
                put_text(
                    &mut frame,
                    "Face",
                    opencv::core::Point::new(face.x, face.y - 10),
                    FONT_HERSHEY_SIMPLEX,
                    0.7,
                    Scalar::new(0.0, 255.0, 0.0, 0.0),
                    2,
                    opencv::imgproc::LINE_8,
                    false,
                )?;
            }
        }

        // Display the processed frame
        imshow(WINDOW_NAME, &frame)?;

        // Wait for a key press (10ms delay). If \'q\' is pressed, exit.
        let key = wait_key(10)?;
        if key == 113 { // \'q\' key
            println!("\n\'q\' pressed. Exiting pipeline.");
            break;
        }
    }

    println!("Pipeline finished.");
    Ok(())
}

fn main() {
    // Ensure the haarcascade file is present for face detection example
    // In a real scenario, this would be handled by build scripts or asset management.
    // For demonstration, we\'ll try to create a dummy file if it doesn\'t exist.
    if !std::path::Path::new(CASCADE_PATH).exists() {
        println!("Creating a dummy cascade file for demonstration. Please replace with a real one for full functionality.");
        // A real haarcascade file is an XML. This is just a placeholder to avoid errors.
        std::fs::write(CASCADE_PATH, "<cascade></cascade>").expect("Unable to write dummy cascade file");
    }

    match run_pipeline() {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
