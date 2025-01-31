use image::DynamicImage;
use ndarray::Array2;
use opencv::{core, core::Vector, highgui, imgproc, prelude::*};

use super::state::ArrowDirection;

pub fn detect_arrow_direction(image: &DynamicImage, debug: bool) -> ArrowDirection {
    // Convertir l'image en niveaux de gris
    let gray_img = image.to_luma8();
    let width = gray_img.width() as usize;
    let height = gray_img.height() as usize;

    // Convertir GrayImage en ndarray
    let array = Array2::from_shape_fn((height, width), |(y, x)| {
        gray_img.get_pixel(x as u32, y as u32)[0]
    });

    // Convertir ndarray en Mat OpenCV
    let mut ocv_mat =
        unsafe { core::Mat::new_rows_cols(height as i32, width as i32, core::CV_8UC1) }.unwrap();
    for y in 0..height {
        let row = array.row(y);
        let mat_row = ocv_mat.at_row_mut::<u8>(y as i32).unwrap();
        mat_row.copy_from_slice(row.as_slice().unwrap());
    }

    // Afficher l'image en niveaux de gris si debug est activé
    if debug {
        highgui::named_window("Gray Image", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("Gray Image", &ocv_mat).unwrap();
        highgui::wait_key(0).unwrap();
    }

    // Binarisation
    let mut binary = core::Mat::default();
    imgproc::threshold(&ocv_mat, &mut binary, 200.0, 255.0, imgproc::THRESH_BINARY).unwrap();

    // Afficher l'image binaire de gris si debug est activé
    if debug {
        highgui::named_window("Binary Image", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("Binary Image", &binary).unwrap();
        highgui::wait_key(0).unwrap();
    }

    // Réduction du bruit
    let kernel = core::Mat::ones(2, 2, core::CV_8U).unwrap();
    let input = binary.clone();
    imgproc::morphology_ex(
        &input,
        &mut binary,
        imgproc::MORPH_OPEN,
        &kernel,
        core::Point::new(-1, -1),
        1,
        core::BORDER_CONSTANT,
        core::Scalar::default(),
    )
    .unwrap();

    // Afficher l'image Cleaned Image de gris si debug est activé
    if debug {
        highgui::named_window("Cleaned Image", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("Cleaned Image", &binary).unwrap();
        highgui::wait_key(0).unwrap();
    }

    // Masquer la moitié droite avec un rectangle
    let cols = binary.cols();
    let rows = binary.rows();
    let rect = core::Rect::new(cols / 2, 0, cols - cols / 2, rows);
    imgproc::rectangle(
        &mut binary,
        rect,
        core::Scalar::all(0.0),
        imgproc::FILLED,
        imgproc::LINE_8,
        0,
    )
    .unwrap();

    // Afficher l'image split si debug est activé
    if debug {
        highgui::named_window("Split Image", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("Split Image", &binary).unwrap();
        highgui::wait_key(0).unwrap();
    }

    // Détection des contours
    let mut contours: Vector<Vector<core::Point>> = Vector::new();
    imgproc::find_contours(
        &binary,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::default(),
    )
    .unwrap();

    // Si debug est activé, afficher les contours
    if debug {
        // Créer une image vide pour dessiner les contours
        let mut drawing = core::Mat::zeros(rows, cols, core::CV_8UC3)
            .unwrap()
            .to_mat()
            .unwrap();

        // Dessiner chaque contour
        for (i, contour) in contours.iter().enumerate() {
            let color = core::Scalar::new(0.0, 255.0, 0.0, 0.0); // Couleur verte
            imgproc::draw_contours(
                &mut drawing,
                &contours,
                i as i32,
                color,
                2,
                imgproc::LINE_8,
                &core::no_array(),
                0,
                core::Point::default(),
            )
            .unwrap();
        }

        // Afficher l'image avec les contours
        highgui::named_window("Contours", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("Contours", &drawing).unwrap();
        highgui::wait_key(0).unwrap();
    }

    let mut best_score = 0;
    let mut best_contour: Option<Vector<core::Point>> = None;
    let mut best_y = 0;

    for contour in contours.iter() {
        let area = imgproc::contour_area(&contour, false).unwrap();

        if area < 70.0 || area > 400.0 {
            continue;
        }

        // debug visualization
        if debug {
            println!("Area: {}", area);
            // Créer une image vide pour dessiner les contours
            let mut drawing = core::Mat::zeros(rows, cols, core::CV_8UC3)
                .unwrap()
                .to_mat()
                .unwrap();

            // List of contours from the current contour
            let mut contours_vec = Vector::<Vector<core::Point>>::new();
            contours_vec.push(contour.clone());

            let color = core::Scalar::new(0.0, 255.0, 0.0, 0.0); // Couleur verte
            imgproc::draw_contours(
                &mut drawing,
                &contours_vec,
                0,
                color,
                2,
                imgproc::LINE_8,
                &core::no_array(),
                0,
                core::Point::default(),
            )
            .unwrap();

            // Afficher l'image avec les contours
            highgui::named_window("Contours", highgui::WINDOW_AUTOSIZE).unwrap();
            highgui::imshow("Contours", &drawing).unwrap();
            highgui::wait_key(0).unwrap();
        }

        let moments = imgproc::moments(&contour, false).unwrap();

        if moments.m00 == 0.0 {
            continue;
        }

        let c_y = (moments.m01 / moments.m00) as i32;
        let mut hu = [0.0; 7];
        imgproc::hu_moments(moments, &mut hu).unwrap();

        let score = calculate_arrow_score(&hu);

        if debug {
            println!("Moments: {:?}", moments);
            println!("Hu Moments: {:?}", hu);
            println!("Score: {}", score);
        }

        if score > 3 && (score >= best_score && c_y > best_y) {
            best_score = score;
            best_y = c_y;
            best_contour = Some(contour.clone());
        }
    }

    match best_contour {
        Some(contour) => determine_direction(&contour, debug),
        None => ArrowDirection::Unknown,
    }
}

fn calculate_arrow_score(hu_moments: &[f64; 7]) -> i32 {
    let mut score = 0;

    if (0.18..0.20).contains(&hu_moments[0]) {
        score += 1;
    }
    if (0.007..0.009).contains(&hu_moments[1]) {
        score += 1;
    }
    if (0.0003..0.0007).contains(&hu_moments[2]) {
        score += 1;
    }
    if (4.0e-05..6.7e-05).contains(&hu_moments[3]) {
        score += 1;
    }
    if (5.0e-09..1.5e-08).contains(&hu_moments[4]) {
        score += 1;
    }
    if (3.6e-06..6.2e-06).contains(&hu_moments[5]) {
        score += 1;
    }
    if (-9.4e-22..2.0e-21).contains(&hu_moments[6]) {
        score += 1;
    }

    score
}

fn determine_direction(contour: &Vector<core::Point>, debug: bool) -> ArrowDirection {
    let rect = imgproc::bounding_rect(&contour).unwrap();
    let mut mask = core::Mat::zeros(rect.height, rect.width, core::CV_8UC1)
        .unwrap()
        .to_mat()
        .unwrap();

    let adjusted_contour: Vector<core::Point> = contour
        .iter()
        .map(|p| core::Point::new(p.x - rect.x, p.y - rect.y))
        .collect();

    let mut contours_vec = Vector::<Vector<core::Point>>::new();
    contours_vec.push(adjusted_contour);

    imgproc::draw_contours(
        &mut mask,
        &contours_vec,
        0,
        core::Scalar::all(255.0),
        imgproc::FILLED,
        8,
        &core::no_array(),
        0,
        core::Point::default(),
    )
    .unwrap();

    let rows = mask.rows();
    let cols = mask.cols();

    // Extraction des bords
    let first_row = mask.row(0).unwrap();
    let last_row = mask.row(rows - 1).unwrap();
    let first_col = mask.col(0).unwrap();
    let last_col = mask.col(cols - 1).unwrap();

    let edges = [
        (first_row, "top"),
        (last_row, "bottom"),
        (first_col, "left"),
        (last_col, "right"),
    ];

    let mut max_count = 0;
    let mut max_side = "";

    for (edge, name) in edges {
        let count = core::count_non_zero(&edge).unwrap();
        if debug {
            println!("Côté {}: {} pixels", name, count);
        }
        if count > max_count {
            max_count = count;
            max_side = name;
        }
    }

    match max_side {
        "right" => ArrowDirection::Left,
        "left" => ArrowDirection::Right,
        "top" => ArrowDirection::Down,
        "bottom" => ArrowDirection::Up,
        _ => ArrowDirection::Unknown,
    }
}
