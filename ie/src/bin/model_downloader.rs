// mby use an http crate in the future like ureq
fn main() {
	std::process::Command::new("curl")
		.args(["https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten", "-o", "./ocr/detection.rten"])
		.output()
		.unwrap();
	
	std::process::Command::new("curl")
		.args(["https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten", "-o", "./ocr/recognition.rten"])
		.output()
		.unwrap();
}