use gtk::Builder;
use gtk::prelude::*;
use crate::ui::ui::Ui;
use std::sync::Arc;
use crate::datatypes::forge::Forge;
use itertools::Itertools;

pub(crate) struct DecorationsPage {
	deco_list: [gtk::FlowBox; 4],
}

impl DecorationsPage {
	pub fn new(builder: &Builder) -> Self {
		let deco_list = [
			builder.get_object("deco lev1").unwrap(),
			builder.get_object("deco lev2").unwrap(),
			builder.get_object("deco lev3").unwrap(),
			builder.get_object("deco lev4").unwrap(),
		];
		DecorationsPage {
			deco_list,
		}
	}

	pub fn show(&self, forge: &Arc<Forge>) {
		for (_, deco) in forge.decorations.iter().sorted_by(|(_, a), (_, b)| { a.name.cmp(&b.name) }) {
			let builder = Ui::get_builder("res/gui/deco box.glade".to_string());
			let deco_box: gtk::Box = builder.get_object("box").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();

			let style = deco_box.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("skillBox");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(deco.name.as_str());
			self.deco_list[deco.size as usize - 1].insert(&deco_box, -1);
		}
	}
}