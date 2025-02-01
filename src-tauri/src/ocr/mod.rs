use image::DynamicImage;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::error::Error;
use std::sync::LazyLock;

const DETECTION_MODEL: &[u8; 2510284] = include_bytes!("text-detection.rten");
const RECOGNITION_MODEL: &[u8; 9716568] = include_bytes!("text-recognition.rten");

static OCR_ENGINE: LazyLock<Result<OcrEngine, Box<dyn std::error::Error + Send + Sync>>> = LazyLock::new(|| {
    let detection_model = Model::load_static_slice(DETECTION_MODEL)?;
    let recognition_model = Model::load_static_slice(RECOGNITION_MODEL)?;

    Ok(OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?)
});

#[derive(Debug)]
pub struct HuntPanelInfos {
    pub step_current: u8,
    pub step_total: u8,
    pub start_x: i8,
    pub start_y: i8,
    pub current_hint: String,
    pub attempts_remaining: u8,
}

/// Performs OCR on a given DynamicImage and returns extracted text.
pub fn ocr(image: &DynamicImage) -> crate::core::error::Result<Vec<String>> {
    let engine = OCR_ENGINE.as_ref().unwrap();

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
