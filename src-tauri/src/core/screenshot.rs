use xcap::Monitor;
use xcap::XCapError;
use image::DynamicImage;
use std::error::Error;
use std::fmt;

/// Représente une région de l'écran avec des coordonnées (x, y, width, height)
pub type ScreenRegion = (i32, i32, u32, u32);

/// Capture une région spécifique de l'écran
pub fn capture_region(region: ScreenRegion) -> Result<DynamicImage, CaptureError> {
    let (x, y, width, height) = region;

    // Trouver le moniteur qui contient le point (x, y)
    let monitor = Monitor::from_point(x, y).map_err(|_| CaptureError::MonitorNotFound)?;

    // Capturer l'image complète du moniteur
    let full_image = monitor.capture_image().map_err(|e| CaptureError::CaptureFailed(e))?;

    // Convertir l'image en DynamicImage pour manipulation
    let mut dynamic_image = DynamicImage::ImageRgba8(full_image);

    // Calculer les coordonnées relatives à la capture du moniteur
    let monitor_x = x - monitor.x();
    let monitor_y = y - monitor.y();

    // Vérifier que la région demandée est valide
    if monitor_x + width as i32 > monitor.width() as i32 || monitor_y + height as i32 > monitor.height() as i32 {
        return Err(CaptureError::InvalidRegion);
    }

    // Extraire la région spécifique
    let region_image = dynamic_image.crop(
        monitor_x as u32,
        monitor_y as u32,
        width,
        height
    );

    Ok(region_image)
}

/// Erreurs possibles lors de la capture d'écran
#[derive(Debug)]
pub enum CaptureError {
    MonitorNotFound,
    InvalidRegion,
    CaptureFailed(XCapError),
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureError::MonitorNotFound => write!(f, "Monitor not found"),
            CaptureError::InvalidRegion => write!(f, "Invalid region specified"),
            CaptureError::CaptureFailed(e) => write!(f, "Capture failed: {}", e),
        }
    }
}

impl Error for CaptureError {}

impl From<XCapError> for CaptureError {
    fn from(err: XCapError) -> Self {
        CaptureError::CaptureFailed(err)
    }
}
