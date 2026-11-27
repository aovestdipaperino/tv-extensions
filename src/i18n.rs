// (C) 2026 - Enzo Lombardi

//! Translations for the texts this crate shows: the CSV editor's menus and
//! dialogs, record-form messages, dual lists, autocomplete fields, JSON
//! dialogs and the log window.
//!
//! The mechanism is turbo-vision's own ([`turbo_vision::i18n`]), so one
//! language setting and one catalogue serve the framework, this crate and
//! your application. This crate looks its texts up under ids that start with
//! `tvx-` and ships them in English (the default and the fallback), Spanish
//! and Catalan. The OK and Cancel buttons are turbo-vision's `button-ok` and
//! `button-cancel`.
//!
//! Nothing changes for an application that never chooses a locale:
//!
//! ```
//! use turbo_vision::i18n;
//!
//! i18n::set_locale("es");
//! tv_extensions::i18n::register();
//! assert_eq!(i18n::tr("tvx-autocomplete-no-matches"), "Sin coincidencias");
//! ```
//!
//! # Order of calls
//!
//! The catalogue is per thread, like turbo-vision's. The first lookup on a
//! thread adds this crate's messages, and [`register`] does it on demand. An
//! application that replaces some of them with
//! [`turbo_vision::i18n::add_translations`] must call [`register`] first,
//! because messages added later win. Views keep the text they were built
//! with, so choose the locale before building them.
//!
//! # Translating
//!
//! [`built_in_source`] returns the English file to start from. Add a
//! translation with `turbo_vision::i18n::add_translations(locale, source)`
//! and check it from a test with [`turbo_vision::i18n::check_translation`]
//! and [`turbo_vision::i18n::check_hot_key_groups`]. The `~x~` hot key of
//! each text must be unique within its menu or dialog; this crate's tests
//! list the groups.

use std::cell::Cell;

use turbo_vision::i18n::{Arg, add_translations, tr_args as tv_tr_args};

/// The catalogues shipped with the crate, as (locale, Fluent source).
const BUILT_IN: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en/tv-extensions.ftl")),
    ("es", include_str!("../locales/es/tv-extensions.ftl")),
    ("ca", include_str!("../locales/ca/tv-extensions.ftl")),
];

thread_local! {
    static REGISTERED: Cell<bool> = const { Cell::new(false) };
}

/// Add this crate's messages to the current thread's catalogue, once.
///
/// Called by every lookup this crate makes, so an application only needs it
/// to put its own replacements after ours (see the module documentation).
/// Calling it again does nothing.
///
/// # Panics
///
/// If a shipped catalogue does not parse. The tests rule that out.
pub fn register() {
    REGISTERED.with(|done| {
        if done.replace(true) {
            return;
        }
        for (locale, source) in BUILT_IN {
            add_translations(locale, source)
                .unwrap_or_else(|e| panic!("built-in tv-extensions {locale}.ftl: {e}"));
        }
    });
}

/// The Fluent source this crate ships for `locale` (`"en"`, `"es"`, `"ca"`),
/// for a translator to start from or for a test to check.
#[must_use]
pub fn built_in_source(locale: &str) -> Option<&'static str> {
    BUILT_IN
        .iter()
        .find(|(tag, _)| *tag == locale)
        .map(|(_, source)| *source)
}

/// The message `id` in the current locale: [`turbo_vision::i18n::tr`] after
/// [`register`].
pub(crate) fn lookup(id: &str) -> String {
    lookup_args(id, &[])
}

/// The message `id` with its placeholders filled.
pub(crate) fn lookup_args(id: &str, args: &[(&str, Arg<'_>)]) -> String {
    register();
    tv_tr_args(id, args)
}

/// `tr!("id")` or `tr!("id", name = value, ..)`: a message of this crate.
macro_rules! tr {
    ($id:expr $(,)?) => {
        $crate::i18n::lookup($id)
    };
    ($id:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::i18n::lookup_args(
            $id,
            &[$((stringify!($name), turbo_vision::i18n::Arg::from($value))),+],
        )
    };
}
pub(crate) use tr;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use turbo_vision::i18n::{
        self, built_in_source as tv_source, check_hot_key_groups, check_translation,
    };

    const LOCALES: [&str; 3] = ["en", "es", "ca"];

    fn english() -> &'static str {
        built_in_source("en").unwrap()
    }

    /// Turbo-vision's file and ours, so groups can name `button-cancel`.
    fn with_buttons(locale: &str) -> String {
        format!(
            "{}\n{}",
            tv_source(locale).unwrap(),
            built_in_source(locale).unwrap()
        )
    }

    #[test]
    fn every_locale_has_the_ids_of_the_english_file() {
        for locale in LOCALES {
            let report = check_translation(english(), built_in_source(locale).unwrap());
            assert!(report.is_ok(), "{locale}: {report:?}");
        }
    }

    /// The texts shown together, by menu and dialog. A letter may repeat
    /// across groups but not within one.
    const GROUPS: &[&[&str]] = &[
        &[
            "tvx-csv-menu-file",
            "tvx-csv-menu-edit",
            "tvx-csv-menu-row",
            "tvx-csv-menu-column",
        ],
        &[
            "tvx-csv-file-new",
            "tvx-csv-file-open",
            "tvx-csv-file-save",
            "tvx-csv-file-save-as",
            "tvx-csv-file-exit",
        ],
        &["tvx-csv-edit-cell", "tvx-csv-edit-rename-column"],
        &["tvx-csv-insert", "tvx-csv-delete"],
        &["button-ok", "button-cancel"],
        &["tvx-csv-open-button", "button-cancel"],
        &[
            "tvx-csv-unsaved-save",
            "tvx-csv-unsaved-discard",
            "button-cancel",
        ],
        &["tvx-csv-delete", "button-cancel"],
    ];

    #[test]
    fn hot_keys_do_not_clash_within_a_menu_or_dialog() {
        for locale in LOCALES {
            let problems = check_hot_key_groups(&with_buttons(locale), GROUPS);
            assert!(problems.is_empty(), "{locale}: {problems:?}");
        }
    }

    /// Every id the code asks for exists, so a typo cannot ship as a bare id.
    #[test]
    fn every_id_used_in_the_source_is_defined() {
        let defined: BTreeSet<&str> = english()
            .lines()
            .filter_map(|l| l.split(" = ").next())
            .filter(|id| id.starts_with("tvx-"))
            .collect();
        let mut used = BTreeSet::new();
        let mut stack = vec![std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("i18n.rs")
                {
                    let text = std::fs::read_to_string(&path).unwrap();
                    let mut rest = text.as_str();
                    while let Some(at) = rest.find("\"tvx-") {
                        let tail = &rest[at + 1..];
                        let end = tail.find('"').unwrap();
                        used.insert(tail[..end].to_string());
                        rest = &tail[end..];
                    }
                }
            }
        }
        assert!(!used.is_empty());
        for id in &used {
            assert!(defined.contains(id.as_str()), "no message '{id}'");
        }
        for id in &defined {
            assert!(
                used.contains(*id),
                "message '{id}' is defined but never used"
            );
        }
    }

    #[test]
    fn english_is_the_default_and_the_fallback() {
        register();
        assert_eq!(i18n::tr("tvx-autocomplete-no-matches"), "No matches");
        i18n::set_locale("de");
        assert_eq!(tr!("tvx-autocomplete-no-matches"), "No matches");
        i18n::set_locale("en");
    }

    #[test]
    fn a_locale_translates_and_arguments_fill_in() {
        i18n::set_locale("ca-ES");
        assert_eq!(
            tr!("tvx-csv-edit-cell-label", row = 3, column = 12),
            "Fila 3, columna 12:"
        );
        i18n::set_locale("en");
        assert_eq!(
            tr!("tvx-csv-edit-cell-label", row = 3, column = 12),
            "Row 3, column 12:"
        );
    }

    #[test]
    fn an_application_replacement_wins_when_added_after_register() {
        register();
        i18n::add_translations("en", "tvx-log-title = Events").unwrap();
        assert_eq!(tr!("tvx-log-title"), "Events");
    }
}
