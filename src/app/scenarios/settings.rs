use crate::core::io2::{styled::{StyledOutput, StyledInput}, Color, Terminal};

pub struct SettingScenarios;

impl SettingScenarios {
  pub fn change_name(t: &mut Terminal) -> String {
    t.input(
      StyledOutput::new()
          .with_text("New name: ")
          .with_color(Color::Cyan),
      Some(StyledInput::new(|input| {
          StyledOutput::new().with_text(format!("Changed name to <{}>.\n", input.to_string().trim()))
      }))
    ).unwrap().to_string()
  }

  pub fn clear_line_after_action(t: &mut Terminal) -> bool {
    SettingsTemplates::toggleable_template(t, "Clear line after action?: ", t.get_settings().clear_line_after_action)
  }

  pub fn use_funny_pastas(t: &mut Terminal) -> bool {
    SettingsTemplates::toggleable_template(t, "Use funny pastas?: ", t.get_settings().funny_pastas)
  }
}

struct SettingsTemplates;

impl SettingsTemplates {
fn toggleable_template(t: &mut Terminal, prompt: &str, flag: bool) -> bool {
  let options = vec!("Yes", "No");

  // true = 1 -> 1 - 1 = 0 => abs(0) = 0 => 1st option
    // false = 0 -> 0 - 1 = -1 => abs(-1) = 1 => 2nd option
    let default_val = i8::abs((flag as i8) - 1) as usize;

    let selected = t.select_one(prompt, &options, default_val);

    match selected {
      0 => true,
      _ => false
    }
  }
}
