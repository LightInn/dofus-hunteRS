use image::DynamicImage;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use regex::Regex;
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

    parse_hunt_panel_text(extracted_text.clone());
    println!("{:?}", extracted_text);
    Ok(extracted_text)
}

#[derive(Debug)]
struct HuntPanelInfos {
    step_current: u8,
    step_total: u8,
    start_x: i8,
    start_y: i8,
    current_hint: String,
    attempts_remaining: u8,
}

fn parse_hunt_panel_text(text: Vec<String>) -> HuntPanelInfos {
    let mut infos = HuntPanelInfos {
        step_current: 0,
        step_total: 0,
        start_x: 0,
        start_y: 0,
        current_hint: String::new(),
        attempts_remaining: 0,
    };

    let step_re = Regex::new(r"STEP:\s*(\d+)/(\d+)").unwrap();
    let coord_re = Regex::new(r"(-?\d+),(-?\d+)").unwrap();
    let attempts_re = Regex::new(r"(\d+)\s*attempts remaining").unwrap();
    let mut hint_candidate = None;
    let mut hint_validated = None;

    for (i, line) in text.iter().enumerate() {
        // Extraction des étapes
        if let Some(caps) = step_re.captures(line) {
            infos.step_current = caps[1].parse().unwrap_or(0);
            infos.step_total = caps[2].parse().unwrap_or(0);
        }
        // Extraction des coordonnées
        else if let Some(caps) = coord_re.captures(line) {
            infos.start_x = caps[1].parse().unwrap_or(0);
            // if the first char is "-", then we need to take 3 chars
            if (caps[2].chars().next().unwrap_or('0')) == '-' {
                infos.start_y = caps[2]
                    .chars()
                    .take(3)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
            } else {
                caps[2]
                    .chars()
                    .take(2)
                    .collect::<String>()
                    .parse()
                    .unwrap_or(0);
            }
        }
        // Détection des tentatives
        else if let Some(caps) = attempts_re.captures(line) {
            infos.attempts_remaining = caps[1].parse().unwrap_or(0);
        }

        // Détection de l'indice avant CONFIRM/attempts
        if line == "CONFIRM" {
            hint_validated = hint_candidate;
        }

        // si len > 2  +  at least 2 words  + tout n'est pas en majuscule (sauf espace) + un seul chiffre max + pas de parenthèse alors c'est un indice potentiel
        if line.len() > 2
            && line.split_whitespace().count() >= 2
            && !line.chars().all(|c| c.is_uppercase() || c.is_whitespace())
            && line.chars().filter(|c| c.is_digit(10)).count() <= 1
            && !line.contains("(")
            && !line.contains("attempts")
            && !line.contains("STEP")
        {
            hint_candidate = Some(line.as_str());
        }
    }

    // Traitement de l'indice
    if let Some(hint) = hint_validated {
        infos.current_hint = hint
            .chars()
            .skip(1) // Supprime le premier caractère
            .collect::<String>()
            .trim_start() // Supprime les espaces initiaux restants
            .to_string();
    }

    println!("{:?}", infos);

    infos
}

// Text: [" TREASURE HUNT", "STEP: 1/3", "O Start [-6,-53]", "Cania Plains (Stontusk Desert)", "IN PROGRESS O", "1 Striped Mushroom", "CONFIRM", "4 attempts remaining", "WAMA", "NWS"]
