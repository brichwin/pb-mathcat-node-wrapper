#![deny(clippy::all)]

use napi_derive::napi;
use libmathcat::interface::*;

#[napi]
pub fn init_mathcat(rules_path: Option<String>) -> Result<String, napi::Error> {
    let path = match rules_path {
        Some(p) => p,
        None => {
            // Try environment variable
            std::env::var("MATHCAT_RULES_DIR")
                .map_err(|_| napi::Error::from_reason(
                    "No rules path provided and MATHCAT_RULES_DIR not set"
                ))?
        }
    };

    set_rules_dir(path.clone())
        .map_err(|e| napi::Error::from_reason(format!("Failed to set rules dir: {}", errors_to_string(&e))))?;

    set_preference("Impairment".to_string(), "Blindness".to_string()).unwrap();
    set_preference("Language".to_string(), "en".to_string()).unwrap();
    set_preference("DecimalSeparator".to_string(), "Auto".to_string()).unwrap();
    set_preference("SpeechOverrides_CapitalLetters".to_string(), "".to_string()).unwrap();
    set_preference("Verbosity".to_string(), "Verbose".to_string()).unwrap();
    set_preference("IntentErrorRecovery".to_string(), "Error".to_string()).unwrap();
    set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();

    set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
    set_preference("TTS".to_string(), "None".to_string()).unwrap();
    set_preference("NavVerbosity".to_string(), "Verbose".to_string()).unwrap();
    set_preference("NavMode".to_string(), "Enhanced".to_string()).unwrap();
    set_preference("MathRate".to_string(), "80".to_string()).unwrap();
    set_preference("CapitalLetters_Beep".to_string(), "true".to_string()).unwrap();
    set_preference("Bookmark".to_string(), "false".to_string()).unwrap();

    Ok(format!("MathCAT initialized with rules at: {}", path))
}

#[napi]
pub fn set_mathcat_preference(key: String, value: String) -> Result<String, napi::Error> {
    set_preference(key.clone(), value.clone())
        .map_err(|e| napi::Error::from_reason(format!("Failed to set preference {}: {}", key, errors_to_string(&e))))?;
    Ok(format!("Preference {} set to {}", key, value))
}

#[napi]
pub fn get_speech_text_from_mathcat(input: String) -> String {

  if let Err(e) = set_mathml(input.to_string()) {
    return format!("-!ERROR!- {}", errors_to_string(&e));
  };

  match get_spoken_text() {
    Ok(speech) => speech,
    Err(e) => format!("-!ERROR!- {}", errors_to_string(&e)),
  }
}

#[napi]
pub fn get_mathcat_version() -> String {
    libmathcat::interface::get_version()
}
