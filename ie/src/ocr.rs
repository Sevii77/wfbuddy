use std::sync::LazyLock;

pub static OCR: LazyLock<Ocr> = LazyLock::new(|| Ocr::new());

pub struct Ocr {
	engine: ocrs::OcrEngine,
}

impl Ocr {
	fn new() -> Self {
		let detection = include_bytes!("../detection.rten");
		let recognition = include_bytes!("../recognition.rten");
		
		let engine = ocrs::OcrEngine::new(ocrs::OcrEngineParams {
			detection_model: Some(rten::Model::load_static_slice(detection).unwrap()),
			recognition_model: Some(rten::Model::load_static_slice(recognition).unwrap()),
			..Default::default()
		}).unwrap();
		
		Self{engine}
	}
	
	pub fn get_text(&self, image: &crate::Image) -> String {
		let data = image.get_bytes();
		let img = ocrs::ImageSource::from_bytes(&data, (image.width(), image.height())).unwrap();
		let input = self.engine.prepare_input(img).unwrap();
		self.engine.get_text(&input).unwrap()
			.split("\n")
			.into_iter()
			.filter(|v| v.len() > 1)
			.collect::<Vec<_>>()
			.join(" ")
	}
}