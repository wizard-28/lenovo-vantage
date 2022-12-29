use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::fl;

#[derive(Debug, strum::Display)]
pub enum FanMode {
    SuperSilent,
    Standard,
    DustCleaning,
    EfficientThermalDissipation,
}

#[derive(Debug)]
pub struct Model {
    vpc: PathBuf,
    conservation_mode: bool,
    fan_mode: FanMode,
}

#[derive(Debug)]
pub enum Input {
    ToggleConservationMode(bool),
    SwitchFanState(FanMode),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Input;
    type Output = ();
    type Widgets = PreferencesWidgets;

    view! {
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: &fl!("battery-group"),

                adw::ActionRow {
                    set_title: &fl!("conservation-title"),
                    set_subtitle: &fl!("conservation-subtitle"),

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,

                        #[watch]
                        #[block_signal(conservation_state_set_handler)]
                        set_active: model.conservation_mode,

                        connect_state_set[sender] => move |switch, _| {
                            sender.input(Self::Input::ToggleConservationMode(switch.is_active()));
                            gtk::Inhibit(false)
                        } @conservation_state_set_handler
                    }
                }
            },
            add = &adw::PreferencesGroup {
                set_title: &fl!("fan-group"),

                adw::ActionRow {
                    set_title: &fl!("super-silent-mode"),

                    #[name = "super_silent_button"]
                    add_suffix = &gtk::CheckButton {
                        #[watch]
                        #[block_signal(super_silent_toggle_handler)]
                        set_active: matches!(model.fan_mode, FanMode::SuperSilent),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(Self::Input::SwitchFanState(FanMode::SuperSilent));
                            }
                        } @super_silent_toggle_handler
                    }
                },
                adw::ActionRow {
                    set_title: &fl!("standard-mode"),

                    #[name = "standard_button"]
                    add_suffix = &gtk::CheckButton {
                        #[watch]
                        #[block_signal(standard_toggle_handler)]
                        set_active: matches!(model.fan_mode, FanMode::Standard),
                        set_group: Some(&super_silent_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(Self::Input::SwitchFanState(FanMode::Standard));
                            }
                        } @standard_toggle_handler
                    }
                },
                adw::ActionRow {
                    set_title: &fl!("dust-cleaning-mode"),

                    #[name = "dust_cleaning_button"]
                    add_suffix = &gtk::CheckButton {
                        #[watch]
                        #[block_signal(dust_cleaning_toggle_handler)]
                        set_active: matches!(model.fan_mode, FanMode::DustCleaning),
                        set_group: Some(&standard_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(Self::Input::SwitchFanState(FanMode::DustCleaning));
                            }
                        } @dust_cleaning_toggle_handler
                    }
                },
                adw::ActionRow {
                    set_title: &fl!("efficient-thermal-dissipation-mode"),

                    #[name = "efficient_thermal_dissipation_button"]
                    add_suffix = &gtk::CheckButton {
                        #[watch]
                        #[block_signal(efficient_thermal_dissipation_toggle_handler)]
                        set_active: matches!(model.fan_mode, FanMode::EfficientThermalDissipation),
                        set_group: Some(&dust_cleaning_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(Self::Input::SwitchFanState(FanMode::EfficientThermalDissipation));
                            }
                        } @efficient_thermal_dissipation_toggle_handler
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let vpc = Path::new("/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00");

        let model = Model {
            vpc: vpc.into(),
            conservation_mode: fs::read_to_string(vpc.join("conservation_mode"))
                .expect("Unable to read conservation mode status")
                .trim()
                .parse::<u8>()
                .unwrap()
                != 0,
            fan_mode: match fs::read_to_string(vpc.join("fan_mode"))
                .expect("Unable to read fan mode status")
                .trim()
                .parse::<u8>()
                .unwrap()
            {
                0 | 133 => FanMode::SuperSilent,
                1 => FanMode::Standard,
                2 => FanMode::DustCleaning,
                3 => FanMode::EfficientThermalDissipation,
                _ => unreachable!(),
            },
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::ToggleConservationMode(switch_on) => {
                let state_string = if switch_on { "on" } else { "off" };
                let state_code = u8::from(switch_on);

                tracing::info!("Turning {state_string} conservation mode",);

                if Command::new("sh")
                    .args([
                        "-c",
                        &format!(
                            "echo {state_code} | pkexec tee {}",
                            self.vpc.join("conservation_mode").display()
                        ),
                    ])
                    .status()
                    .unwrap()
                    .success()
                {
                    tracing::info!("Successfully toggled {state_string} conservation mode");
                    self.conservation_mode = switch_on;
                } else {
                    tracing::error!("Unable to toggle {state_string} conservation mode");
                }
            },
            Self::Input::SwitchFanState(state) => {
                tracing::info!("{}", &format!("Switching to {state} fan"));

                let state_code = match state {
                    FanMode::SuperSilent => 0,
                    FanMode::Standard => 1,
                    FanMode::DustCleaning => 2,
                    FanMode::EfficientThermalDissipation => 4,
                };

                if Command::new("sh")
                    .args([
                        "-c",
                        &format!(
                            "echo {state_code} | pkexec tee {}",
                            self.vpc.join("fan_mode").display()
                        ),
                    ])
                    .status()
                    .unwrap()
                    .success()
                {
                    tracing::info!("{}", &format!("Successfully switched to {state} fan"));
                    self.fan_mode = state;
                } else {
                    tracing::error!("{}", &format!("Unable to switch to {state} fan"));
                }
            },
        };
    }
}
