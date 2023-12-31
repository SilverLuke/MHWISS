use std::rc::Rc;
use gtk::{Builder, FlowBoxChild, SizeGroupMode};
use gtk::prelude::*;
use itertools::Itertools;
use crate::data::{
	db_storage::Storage,
	dyn_storage::DynamicStorage,
};
use crate::ui::get_builder;

pub(crate) struct DecorationsPage {
	deco_list: [gtk::FlowBox; 4],
	search_bar: gtk::SearchEntry,
	quantity_btn: gtk::SpinButton,
	set_quantity_btn: gtk::Button,
}

impl DecorationsPage {
	pub fn new(builder: &Builder) -> Self {
		let deco_list = [
			builder.object("deco lev1").unwrap(),
			builder.object("deco lev2").unwrap(),
			builder.object("deco lev3").unwrap(),
			builder.object("deco lev4").unwrap(),
		];
		let page = DecorationsPage {
			deco_list,
			search_bar: builder.object("decoration search bar").unwrap(),
			quantity_btn: builder.object("quantity deco").unwrap(),
			set_quantity_btn: builder.object("set quantity deco").unwrap(),
		};
		page.connect_signals();
		page
	}
	fn connect_signals(&self) {
		// Search functionality
		let decos = self.deco_list.clone();
		self.search_bar.connect_search_changed(move |_sb| {
			for flowbox in &decos {
				flowbox.invalidate_filter();
			}
		});

		let search_bar = self.search_bar.clone();
		let filter = move |child: &FlowBoxChild| {
			let gtkbox: gtk::Box = (child.child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
			for c in gtkbox.children() {
				if let Some(label) = c.downcast_ref::<gtk::Label>() {
					let deco_name = label.text();
					return deco_name.to_lowercase().contains(search_bar.text().to_lowercase().as_str());
				}
			}
			false
		};
		for flowbox in &self.deco_list {
			flowbox.set_filter_func(Some(Box::new(filter.clone())));
		}

		// Set quantity btn
		let quantity = self.quantity_btn.clone();
		let flow_boxes = self.deco_list.clone();
		self.set_quantity_btn.connect_clicked(move |_btn| {
			let quantity = quantity.value();
			let setter = |w: &gtk::Widget| {
				let gtk_box: gtk::Box = ((w.downcast_ref::<gtk::FlowBoxChild>().unwrap()).child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
				for c in gtk_box.children() {
					if let Some(spin) = c.downcast_ref::<gtk::SpinButton>() {
						spin.set_value(quantity);
						return;
					}
				}
			};
			for flowbox in flow_boxes.iter() {
				flowbox.foreach(setter.clone());
			}
		});
	}

	pub fn show(&self, storage: &Rc<Storage>, dynamic_storage: &Rc<DynamicStorage>) {
		let size_group: gtk::SizeGroup = gtk::SizeGroup::new(SizeGroupMode::Both);
		for deco in storage.decorations.iter().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = get_builder("res/gui/deco box.glade".to_string());
			let deco_flowbox_child: gtk::FlowBoxChild = builder.object("flowbox").unwrap();
			let name: gtk::Label = builder.object("name").unwrap();

			let style = deco_flowbox_child.style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("FlowBoxSkill");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(deco.name.as_str());
			size_group.add_widget(&deco_flowbox_child);
			self.deco_list[deco.size as usize - 1].insert(&deco_flowbox_child, -1);
		}
	}
}