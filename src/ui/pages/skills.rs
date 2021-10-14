use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::prelude::BuilderExtManual;
use itertools::Itertools;

use crate::ui::get_builder;
use std::sync::Arc;
use gtk::{Builder, FlowBoxChild, SizeGroupMode};
use crate::data::db_storage::Storage;
use crate::data::db_types::skill::SkillLevel;
use crate::data::dyn_storage::DynamicStorage;

pub(crate) struct SkillsPage {
	skill_list: gtk::FlowBox,
	armor_set_skill_list: gtk::FlowBox,
	search_bar: gtk::SearchEntry,
	reset_btn: gtk::Button,
}

impl SkillsPage {
	pub fn new(builder: &Builder, dynamic_storage: &Rc<DynamicStorage>) -> Self {
		let skill_list = builder.object("skill list").expect("UI do not contains \"skill list\"");
		let armor_set_skill_list = builder.object("skill set list").expect("UI do not contains \"skill set list\"");
		let search_bar = builder.object("skill search bar").expect("UI do not contains \"skill search bar\"");
		let reset_btn = builder.object("reset constraints btn").expect("UI do not contains \"reset constraints btn\"");
		let page = SkillsPage {
			skill_list,
			armor_set_skill_list: armor_set_skill_list,
			search_bar,
			reset_btn,
		};
		page.connect_signals(dynamic_storage);
		page
	}

	fn connect_signals(&self, em: &Rc<DynamicStorage>) {
		let skill_list = self.skill_list.clone();
		let armorset_skill_list = self.armor_set_skill_list.clone();
		self.search_bar.connect_search_changed(move |_sb| {
			skill_list.invalidate_filter();
			armorset_skill_list.invalidate_filter();
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
		self.skill_list.set_filter_func(Some(Box::new(filter.clone())));
		self.armor_set_skill_list.set_filter_func(Some(Box::new(filter)));

		let dynamic_storage_copy = Rc::clone(em);
		let skill_list = self.skill_list.clone();
		let armorset_skill_list = self.armor_set_skill_list.clone();
		self.reset_btn.connect_clicked(move |_btn| {
			println!("RESET");
			dynamic_storage_copy.clean_constrains();
			let resetter = |w: &gtk::Widget| {
				let gtkbox: gtk::Box = ((w.downcast_ref::<FlowBoxChild>().unwrap()).child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
				for c in gtkbox.children() {
					if let Some(spin) = c.downcast_ref::<gtk::SpinButton>() {
						spin.clone().set_value(0.0);
						return;
					}
				}
			};
			skill_list.foreach(resetter.clone());
			armorset_skill_list.foreach(resetter);
		});
	}

	pub fn show(&self, storage: &Rc<Storage>, dynamic_storage: &Rc<DynamicStorage>) {
		let size_group: gtk::SizeGroup = gtk::SizeGroup::new(SizeGroupMode::Both);
		for skill in storage.skills.iter().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = get_builder("res/gui/skill box.glade".to_string());
			let skill_flowbox: gtk::FlowBoxChild = builder.object("flowbox").unwrap();
			let name: gtk::Label = builder.object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.object("level").unwrap();

			let style = skill_flowbox.style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("FlowBoxSkill");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(skill.name.as_str());
			name.set_tooltip_text(Some(skill.description.as_str()));
			adjustment.set_upper(skill.max_level as f64);

			let dynamic_storage_copy = Rc::clone(dynamic_storage);
			let skill_copy = Arc::clone(skill);
			level.connect_value_changed(move |lev| {
				let skill_level = SkillLevel::new(Arc::clone(&skill_copy), lev.value() as u8);
				dynamic_storage_copy.set_constraint(skill_level);
			});
			size_group.add_widget(&skill_flowbox);

			self.skill_list.insert(&skill_flowbox, -1);
		}

		for skill in storage.set_skills.iter().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = get_builder("res/gui/skill box.glade".to_string());
			let skill_flowbox: gtk::FlowBoxChild = builder.object("flowbox").unwrap();
			let name: gtk::Label = builder.object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.object("adjustment").unwrap();
			let _level: gtk::SpinButton = builder.object("level").unwrap();

			let style = skill_flowbox.style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("FlowBoxSkill");

			name.set_text(skill.name.as_str());
			adjustment.set_upper(skill.get_max() as f64);
			/*
			let app = Rc::clone(&application);
			let skill_copy = Arc::clone(skill);

			TODO dependency skills?
			level.connect_value_changed(move |lev| {
				let skill_level = SkillLevel::new(skill_copy, lev.get_value() as u8);
				app.engine_manager.add_constraint(skill_level);
			});
			*/
			size_group.add_widget(&skill_flowbox);
			self.armor_set_skill_list.insert(&skill_flowbox, -1);
		}
	}
}