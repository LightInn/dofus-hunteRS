use crate::core::error::Error;
use crate::core::error::Result;
use image::{DynamicImage, GrayImage, Luma};
use regex::Regex;

use crate::ocr::ocr;

#[derive(Debug)]
pub struct HuntPanelInfos {
    pub step_current: u8,
    pub step_total: u8,
    pub start_x: i8,
    pub start_y: i8,
    pub current_hint: String,
    pub attempts_remaining: u8,
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
        if line.contains("CONFIRM") {
            hint_validated = hint_candidate;
        }

        // si len > 2  +  at least 2 words  + tout n'est pas en majuscule (sauf espace) + un seul chiffre max + pas de parenthèse alors c'est un indice potentiel
        if line.len() > 3
            // && line.split_whitespace().count() >= 2
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
        // on supprime le premier caractère

        infos.current_hint = hint
            .chars()
            .skip(1) // Supprime le premier caractère
            .collect::<String>()
            .trim_start() // Supprime les espaces initiaux restants
            .to_string();

        // on supprimes les mots entiers lorsqu'ils sont en majuscule
        let mut hint_words = infos.current_hint.split_whitespace().collect::<Vec<&str>>();
        let mut hint_words_filtered = Vec::new();
        for word in hint_words.iter() {
            if !word.chars().all(|c| c.is_uppercase()) {
                hint_words_filtered.push(*word);
            }
        }
        infos.current_hint = hint_words_filtered.join(" ");
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

pub fn ocr_hunt_panel(image: &DynamicImage) -> Result<HuntPanelInfos> {
    let extracted_text = ocr(image)?;

    println!("{:?}", extracted_text);

    let infos = parse_hunt_panel_text(extracted_text.clone());

    println!("{:?}", infos);
    Ok(infos)
}

pub fn ocr_coordinates(image: &DynamicImage) -> Result<Option<(i8, i8)>> {
    // let binary_img = binarize_dynamic_image(&image, 60);

    // show image
    // binary_img.save("binary_img.png")?;

    let extracted_text = ocr(image)?;

    println!("{:?}", extracted_text);
    let parsed_coordinates = parse_coordinates(extracted_text.clone());
    println!("{:?}", parsed_coordinates);

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
