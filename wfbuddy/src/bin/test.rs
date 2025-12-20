use std::{fs::File, io::BufReader};

fn main() {
	let mut args = std::env::args();
	args.next();
	let args = args.collect::<Vec<_>>();
	
	match args[0].as_str() {
		"relicreward_rewards" => {
			let data = data::Data::populated();
			let img = load_image(&args[1]);
			let out = ie::screen::relicreward::get_rewards(img.as_image())
				.into_iter()
				.map(|v| (data.find_item_name(&v.name), v.owned))
				.collect::<Vec<_>>();
			println!("relicreward_rewards {out:#?}");
		}
		
		"relicreward_picked" => {
			let img = load_image(&args[1]);
			let out = ie::screen::relicreward::get_selected(img.as_image());
			println!("relicreward_picked {out:#?}");
		}
		
		v => println!("unknown test {v}")
	}
}

fn load_image(path: &str) -> ie::OwnedImage {
	let mut reader = png::Decoder::new(BufReader::new(File::open(path).unwrap()));
	reader.set_transformations(png::Transformations::all());
	let mut reader = reader.read_info().unwrap();
	let mut buf = vec![0u8; reader.output_buffer_size().unwrap()];
	let info = reader.next_frame(&mut buf).unwrap();
	let mut img = ie::OwnedImage::from_rgba(info.width as usize, &buf[..info.buffer_size()]);
	img.resize_h(1080);
	img
}