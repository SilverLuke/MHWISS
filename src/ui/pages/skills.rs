use crate::ui::ui::Ui;
use gtk::{Builder, WidgetExt, CssProviderExt, StyleContextExt, LabelExt, AdjustmentExt, FlowBoxExt};
use gtk::prelude::BuilderExtManual;
use std::rc::Rc;
use crate::datatypes::forge::Forge;
use std::sync::Arc;
use itertools::Itertools;
use gio::prelude::*;
use gtk::prelude::*;

pub(crate) struct SkillsPage {
	skill_list: gtk::FlowBox,
	armorset_skill_list: gtk::FlowBox,
}

impl SkillsPage {
	pub fn new(builder: &Builder) -> Self {
		let skill_list = builder.get_object("skill list").unwrap();
		let armorset_skill_list = builder.get_object("skill set").unwrap();
		SkillsPage {
			skill_list,
			armorset_skill_list,
		}
	}

	pub fn show(&self, application: Rc<Ui>, forge: &Arc<Forge>) {  // TODO: Add skill dependecy
		for skill in forge.skills.values().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = Ui::get_builder("gui/skill box.glade".to_string());
			let skill_box: gtk::Box = builder.get_object("box").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.get_object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.get_object("level").unwrap();

			let style = skill_box.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("skillBox");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(skill.name.as_str());
			name.set_tooltip_text(Some(skill.description.as_str()));
			adjustment.set_upper(skill.max_level as f64);

			let app = Rc::clone(&application);
			// let skill_req = Rc::clone(&skill);
			let id = skill.id;
			level.connect_value_changed(move |lev| {
				app.searcher.add_skill_requirement(id, lev.get_value() as u8);
			});

			self.skill_list.insert(&skill_box, -1);
		}

		for skill in forge.set_skills.values().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = Ui::get_builder("gui/skill box.glade".to_string());
			let skill_box: gtk::Box = builder.get_object("box").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.get_object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.get_object("level").unwrap();

			let style = skill_box.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("skillBox");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(skill.name.as_str());
			adjustment.set_upper(skill.get_max() as f64);

			let app = Rc::clone(&application);
			let id = skill.id;
			level.connect_value_changed(move |lev| {
				app.searcher.add_skill_requirement(id, lev.get_value() as u8);
			});

			self.armorset_skill_list.insert(&skill_box, -1);
		}
	}
}