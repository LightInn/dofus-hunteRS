use crate::composent::config::ApiConfig;
use crate::core::error::Error;
use crate::core::error::Result;
use crate::ocr::ocr;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use image::{DynamicImage, GrayImage, Luma};
use regex::Regex;
use strsim::{jaro_winkler, levenshtein};

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
    let mut matched_indices = Vec::new();
    let mut available_indices = indices.clone();
    available_indices.push("Drheller".to_string());
    let mut first_open = true;
    let mut first_close = true;

    let cleaned_text = ocr_text
        .iter()
        .map(|text| {
            text.trim()
                .replace("\"", "")
                .replace("?", "")
                .replace("*", "")
        })
        .map(|text| {
            if first_close && text.contains(")") {
                first_close = false;
                return "".to_string();
            } else {
                text
            }
        })
        .map(|text| {
            if first_open && text.contains("(") {
                first_open = false;
                return "".to_string();
            } else {
                text
            }
        })
        .filter(|text| !text.is_empty())
        .collect::<Vec<String>>();


    let cleaned_text = cleaned_text
        .iter()
        .map(|s| {
            let mut words = s.split_whitespace().collect::<Vec<&str>>();
            // keep only words with alphabetic characters
            words.retain(|word| word.to_string().chars().all(char::is_alphabetic));
            // keep only words with lowercase characters
            words.retain(|word| !word.chars().all(char::is_uppercase));
            words.join(" ")
        })
        .collect::<Vec<String>>();

    // Convertir les éléments OCR en une séquence de mots
    let mut ocr_words: Vec<String> = cleaned_text
        .iter()
        .flat_map(|text| text.split_whitespace())
        .filter(|word| word.len() >= 3)
        .map(|word| word.to_string())
        .collect();

    if ocr_words.len() > 3 {
        ocr_words = ocr_words.split_off(1);
    }

    // println!("ocr_words: {:?}", ocr_words);

    // Parcourir les mots OCR dans leur ordre original
    let mut analyze_words = ocr_words.clone();
    let mut current_phrase = String::new();
    let mut not_finished = true;

    while not_finished {
        let cloned_analyze_words = analyze_words.clone();
        let mut adjust = 0;

        for (index, word) in cloned_analyze_words.iter().enumerate() {
            current_phrase.push_str(word);
            current_phrase.push(' ');

            // println!("current_phrase: {:?}", current_phrase);

            let best_match = best_match(&current_phrase, &available_indices, 12, 0.85);

            // Si un match satisfaisant est trouvé
            if let Some((best_index)) = best_match {
                // Seuil de correspondance
                matched_indices.push(best_index.clone());
                // available_indices.retain(|i| *i != best_index);
                current_phrase = String::new(); // Réinitialiser la phrase courante
                analyze_words = analyze_words.split_off(index + 1 - adjust);
                // println!("analyze_words: {:?}", analyze_words);
                adjust = index + 1;
            }
            // Limiter la longueur de la phrase courante pour éviter des matchs trop larges
            if current_phrase.split_whitespace().count() > 5 {
                // println!("Tronc");
                // remove first word of
                analyze_words = analyze_words.split_off(1);
                current_phrase = String::new();

                if analyze_words.len() == 0 {
                    not_finished = false;
                }
                break;
            }
            if index == ocr_words.len() - 1 {
                not_finished = false;
            }
        }
    }

    matched_indices
}

fn best_match<'a>(
    query: &'a str,
    texts: &'a [String],
    lev_threshold: usize,
    jaro_threshold: f64,
) -> Option<&'a String> {
    if query.len() <= 20 {
        texts
            .iter()
            .map(|text| (text, jaro_winkler(query, text)))
            .filter(|&(_, score)| {
                if score >= jaro_threshold {
                    // println!("score: {:?}", score);
                }
                score >= jaro_threshold
            }) // On garde les scores élevés
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) // On prend le meilleur
            // check if the best match have the same number of words as the query
            .filter(|&(best_match, _)| {
                let query_words = query.split_whitespace().count();
                let best_match_words = best_match.split_whitespace().count();
                query_words == best_match_words
            })
            .map(|(best_match, _)| best_match)
    } else {
        // Pour les phrases longues, Levenshtein est plus efficace
        texts
            .iter()
            .map(|text| (text, levenshtein(query, text)))
            .filter(|&(test, distance)| {
                if distance <= lev_threshold {
                    // println!("distance: {:?} - test: {:?}", distance, test);
                }
                distance <= lev_threshold
            })
            .min_by_key(|&(_, distance)| distance)
            .map(|(best_match, _)| best_match)
    }
}
