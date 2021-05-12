use std::rc::Rc;

use gio::prelude::*;
use gtk::{AdjustmentExt, Builder, CssProviderExt, FlowBoxChild, FlowBoxExt, LabelExt, SizeGroupMode, StyleContextExt, WidgetExt};
use gtk::prelude::*;
use gtk::prelude::BuilderExtManual;
use itertools::Itertools;

use crate::engines::EnginesManager;
use crate::ui::{Ui, get_builder};

pub(crate) struct SkillsPage {
	skill_list: gtk::FlowBox,
	armorset_skill_list: gtk::FlowBox,
	search_bar: gtk::SearchEntry,
	reset_btn: gtk::Button,
}

impl SkillsPage {
	pub fn new(builder: &Builder, em: &Rc<EnginesManager>) -> Self {
		let skill_list = builder.get_object("skill list").unwrap();
		let armorset_skill_list = builder.get_object("skill set list").unwrap();
		let search_bar = builder.get_object("skill search bar").unwrap();
		let reset_btn = builder.get_object("reset constrains btn").unwrap();
		let page = SkillsPage {
			skill_list,
			armorset_skill_list,
			search_bar,
			reset_btn,
		};
		page.connect_signals(em);
		page
	}

	fn connect_signals(&self, em: &Rc<EnginesManager>) {
		let skill_list = self.skill_list.clone();
		let armorset_skill_list = self.armorset_skill_list.clone();
		self.search_bar.connect_search_changed(move |_sb| {
			skill_list.invalidate_filter();
			armorset_skill_list.invalidate_filter();
		});

		let search_bar = self.search_bar.clone();
		let filter = move |child: &FlowBoxChild| {
			let gtkbox: gtk::Box = (child.get_child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
			for c in gtkbox.get_children() {
				if let Some(label) = c.downcast_ref::<gtk::Label>() {
					let skill_name = label.get_text();
					return skill_name.to_lowercase().contains(search_bar.get_text().to_lowercase().as_str());
				}
			}
			false
		};
		self.skill_list.set_filter_func(Some(Box::new(filter.clone())));
		self.armorset_skill_list.set_filter_func(Some(Box::new(filter)));

		let em_ref = Rc::clone(em);
		let skill_list = self.skill_list.clone();
		let armorset_skill_list = self.armorset_skill_list.clone();
		self.reset_btn.connect_clicked(move |_btn| {
			println!("RESET");
			em_ref.clean_constrains();
			let resetter = |w: &gtk::Widget| {
				let gtkbox: gtk::Box = ((w.downcast_ref::<gtk::FlowBoxChild>().unwrap()).get_child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
				for c in gtkbox.get_children() {
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

	pub fn show(&self, application: &Rc<Ui>) {  // TODO: Add skill dependecy
		let forge = &application.forge;
		let size_group : gtk::SizeGroup = gtk::SizeGroup::new(SizeGroupMode::Both);
		for skill in forge.skills.values().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = get_builder("res/gui/skill box.glade".to_string());
			let skill_flowbox: gtk::FlowBoxChild = builder.get_object("flowbox").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.get_object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.get_object("level").unwrap();


			let style = skill_flowbox.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("FlowBoxSkill");  // TODO: Better implementation using glades => Add this feature in glade


			name.set_text(skill.name.as_str());
			name.set_tooltip_text(Some(skill.description.as_str()));
			adjustment.set_upper(skill.max_level as f64);

			let app = Rc::clone(&application);
			let id = skill.id;
			level.connect_value_changed(move |lev| {
				app.engine_manager.add_constraint(id, lev.get_value() as u8);
			});
			size_group.add_widget(&skill_flowbox);

			self.skill_list.insert(&skill_flowbox, -1);
		}

		for skill in forge.set_skills.values().sorted_by(|a, b| { a.name.cmp(&b.name) }) {
			let builder = get_builder("res/gui/skill box.glade".to_string());
			let skill_flowbox: gtk::FlowBoxChild = builder.get_object("flowbox").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.get_object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.get_object("level").unwrap();

			let style = skill_flowbox.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("res/gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("FlowBoxSkill");

			name.set_text(skill.name.as_str());
			adjustment.set_upper(skill.get_max() as f64);

			let app = Rc::clone(&application);
			let id = skill.id;
			level.connect_value_changed(move |lev| {
				app.engine_manager.add_constraint(id, lev.get_value() as u8);
			});
			size_group.add_widget(&skill_flowbox);
			self.armorset_skill_list.insert(&skill_flowbox, -1);
		}
	}
}