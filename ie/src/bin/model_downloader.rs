// mby use an http crate in the future like ureq
fn main() {
	std::process::Command::new("wget")
		.args(["-O", "./ocr/detection.mnn", "https://github.com/zibo-chen/rust-paddle-ocr/raw/2ef84ebbc1d07d8ea6296340e3c05496bd7dfe8e/models/PP-OCRv5_mobile_det_fp16.mnn"])
		.output()
		.unwrap();
	
	std::process::Command::new("wget")
		.args(["-O", "./ocr/latin_recognition.mnn", "https://github.com/zibo-chen/rust-paddle-ocr/raw/2ef84ebbc1d07d8ea6296340e3c05496bd7dfe8e/models/latin_PP-OCRv5_mobile_rec_infer.mnn"])
		.output()
		.unwrap();
	
	std::process::Command::new("wget")
		.args(["-O", "./ocr/latin_charset.txt", "https://github.com/zibo-chen/rust-paddle-ocr/raw/2ef84ebbc1d07d8ea6296340e3c05496bd7dfe8e/models/ppocr_keys_latin.txt"])
		.output()
		.unwrap();
}