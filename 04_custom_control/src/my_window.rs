use winsafe::{prelude::*, co, gui};
use winsafe::{ErrResult, HINSTANCE, IdIdiStr, POINT, SIZE};

use crate::click_board::ClickBoard;

#[derive(Clone)]
pub struct MyWindow {
	wnd:         gui::WindowMain,
	click_board: ClickBoard,
}

impl MyWindow {
	pub fn new() -> ErrResult<MyWindow> {
		let hinstance = HINSTANCE::GetModuleHandle(None)?;

		let wnd = gui::WindowMain::new(
			gui::WindowMainOpts {
				title: "Custom control".to_owned(),
				class_icon: hinstance.LoadIcon(IdIdiStr::Id(101))?,
				size: SIZE::new(300, 150),
				style: gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX, // add a minimize button
				..Default::default()
			},
		);

		let click_board = ClickBoard::new(
			&wnd,
			POINT::new(10, 10),
			SIZE::new(280, 130),
		)?;

		let mut new_self = Self { wnd, click_board };
		new_self.events();
		Ok(new_self)
	}

	pub fn run(&self) -> ErrResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&mut self) {
		self.click_board.on_click({ // click event of our custom control
			let wnd = self.wnd.clone();
			move |num_points| {
				wnd.hwnd().SetWindowText(&format!("Points: {}", num_points))?;
				Ok(())
			}
		});
	}
}
