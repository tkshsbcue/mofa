//! Cached regex patterns for prompt variable substitution.
//!
//! Per project standards: regex objects with high compilation costs MUST be
//! cached using LazyLock or OnceLock.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Cached regex for template variable placeholders: `{var_name}`
pub(crate) static VARIABLE_PLACEHOLDER_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\{(\w+)\}").unwrap());

/// Thread-safe cache for user-defined validation regex patterns.
///
/// Avoids recompiling the same pattern string on every
/// `PromptVariable::validate()` call.
pub(crate) static VALIDATION_REGEX_CACHE: LazyLock<Mutex<HashMap<String, regex::Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
