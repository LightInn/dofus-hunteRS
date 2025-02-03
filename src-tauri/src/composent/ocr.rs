use crate::composent::config::ApiConfig;
use crate::core::error::Error;
use crate::core::error::Result;
use crate::ocr::ocr;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use image::{DynamicImage, GrayImage, Luma};
use regex::Regex;

#[derive(Debug)]
pub struct HuntPanelInfos {
    pub step_current: u8,
    pub step_total: u8,
    pub start_x: i8,
    pub start_y: i8,
    pub current_hint: String,
    pub attempts_remaining: u8,
}

fn parse_hunt_panel_text(text: Vec<String>, config: ApiConfig) -> HuntPanelInfos {
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

    let mut hint_validated = None;

    for (_i, line) in text.iter().enumerate() {
        // Extraction des étapes
        if let Some(caps) = step_re.captures(line) {
            infos.step_current = caps[1].parse().unwrap_or(0);
            infos.step_total = caps[2].parse().unwrap_or(0);
        }
        // Extraction des coordonnées
        else if let Some(caps) = coord_re.captures(line) {
            infos.start_x = caps[1].parse().unwrap_or(0);
            // if the first char is "-", then we need to take 3 chars
            if caps[2]
                .as_bytes()
                .first()
                .map(|&c| c as char)
                .unwrap_or('0')
                == '-'
            {
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
    }

    let indices = crate::composent::api::get_hints_from_api(config).unwrap();
    let results = ocr_order_fuzzy_search(&text, &indices);
    println!("Indices trouvés : {:?}", results);

    // hint_validated is last hint in results
    hint_validated = results.last().map(|s| s.to_string());

    // Traitement de l'indice
    if let Some(hint) = hint_validated {
        // on supprime le premier caractère
        infos.current_hint = hint.to_string();
    }

    infos
}

fn parse_coordinates(text: Vec<String>) -> Result<Option<(i8, i8)>> {
    let coord_re = Regex::new(r"(-?\d+),\s*(-?\d+)")?;

    for line in text.iter() {
        if let Some(caps) = coord_re.captures(line) {
            if let (Ok(x), Ok(y)) = (caps[1].parse::<i8>(), caps[2].parse::<i8>()) {
                return Ok(Some((x, y)));
            }
        }
    }

    Ok(None) // Aucun match trouvé
}

pub fn ocr_hunt_panel(image: &DynamicImage, config: ApiConfig) -> Result<HuntPanelInfos> {
    let extracted_text = ocr(image)?;

    println!("{:?}", extracted_text);

    let infos = parse_hunt_panel_text(extracted_text.clone(), config);

    println!("{:?}", infos);
    Ok(infos)
}

pub fn ocr_coordinates(image: &DynamicImage) -> Result<Option<(i8, i8)>> {
    // let binary_img = binarize_dynamic_image(&image, 60);

    // show image
    // binary_img.save("binary_img.png")?;

    let extracted_text = ocr(image)?;

    // println!("{:?}", extracted_text);
    let parsed_coordinates = parse_coordinates(extracted_text.clone());
    println!("parse coord {:?}", parsed_coordinates);

    // let infos = parse_hunt_panel_text(extracted_text.clone());
    //
    //     ["-4, -24?36%"]
    //     ["-5,-24- Lev"]
    //     ["-78, -41-Lel"]
    //     ["-22, 34-Lev"]
    //     ["13,27 -Level"]

    Ok(parsed_coordinates?)
}

fn binarize_dynamic_image(img: &DynamicImage, threshold: u8) -> DynamicImage {
    let grayscale = img.to_luma8(); // Convertit en niveaux de gris
    let (width, height) = grayscale.dimensions();

    let mut binary_img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = grayscale.get_pixel(x, y);
            let Luma([luma]) = *pixel;
            let new_pixel = if luma > threshold { 255 } else { 0 };
            binary_img.put_pixel(x, y, Luma([new_pixel]));
        }
    }

    DynamicImage::ImageLuma8(binary_img) // Convertir de GrayImage en DynamicImage
}

fn ocr_order_fuzzy_search(ocr_text: &Vec<String>, indices: &Vec<String>) -> Vec<String> {
    let matcher = SkimMatcherV2::default();
    let mut matched_indices = Vec::new();
    let mut available_indices = indices.clone();

    // Convertir les éléments OCR en une séquence de mots
    let ocr_words: Vec<String> = ocr_text
        .iter()
        .map(|text| text.trim().replace("\"", ""))
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|s| s.split_whitespace().map(|w| w.to_string()))
        .collect();

    // Parcourir les mots OCR dans leur ordre original
    let mut current_phrase = String::new();
    for (index, word) in ocr_words.iter().enumerate() {
        current_phrase.push_str(word);
        current_phrase.push(' ');

        // Vérifier les indices disponibles
        let best_match = available_indices
            .iter()
            .filter_map(|idx| {
                matcher
                    .fuzzy_match(&current_phrase, idx)
                    .map(|score| (idx.clone(), score))
            })
            .max_by_key(|&(_, score)| score);

        // Si un match satisfaisant est trouvé
        if let Some((best_index, score)) = best_match {
            if score > 50 {
                // Seuil de correspondance
                matched_indices.push(best_index.clone());
                available_indices.retain(|i| *i != best_index);
                current_phrase = String::new(); // Réinitialiser la phrase courante
            }
        }

        // Limiter la longueur de la phrase courante pour éviter des matchs trop larges
        if current_phrase.split_whitespace().count() > 5 {
            current_phrase = current_phrase
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");
        }
    }

    matched_indices
}
