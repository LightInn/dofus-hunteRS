use image::DynamicImage;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::error::Error;
use std::path::PathBuf;
use std::sync::LazyLock;

static OCR_ENGINE: LazyLock<Result<OcrEngine, Box<dyn Error + Send + Sync>>> =
    LazyLock::new(|| {
        let detection_model_path = file_path("ocr_models/text-detection.rten");
        let rec_model_path = file_path("ocr_models/text-recognition.rten");

        let detection_model = Model::load_file(detection_model_path)?;
        let recognition_model = Model::load_file(rec_model_path)?;

        Ok(OcrEngine::new(OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        })?)
    });

/// Given a file path relative to the crate root, return the absolute path.
fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

/// Performs OCR on a given DynamicImage and returns extracted text.
pub fn ocr(image: &DynamicImage) -> Result<Vec<String>, Box<dyn Error>> {
    let engine = OCR_ENGINE.as_ref().map_err(|e| e.to_string())?;

    let img = image.to_rgb8();
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    let word_rects = engine.detect_words(&ocr_input)?;
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    let extracted_text: Vec<String> = line_texts
        .iter()
        .flatten()
        .filter(|l| l.to_string().len() > 1)
        .map(|l| l.to_string())
        .collect();

    Ok(extracted_text)
}
