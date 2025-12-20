fn main() {
	for window in xcap::Window::all().unwrap() {
		println!("{:?} {:?}", window.app_name(), window.title());
	}
}