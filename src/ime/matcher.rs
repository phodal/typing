/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ime::event::{KeyEvent, KeyEventReceiver, KeyModifier};

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Mapping;
use std::path::PathBuf;

#[derive(Debug, Serialize, Clone)]
pub struct Match {
    pub triggers: Vec<String>,
    pub content: MatchContentType,
    pub word: bool,
    pub passive_only: bool,
    pub propagate_case: bool,
    pub force_clipboard: bool,

    // Automatically calculated from the triggers, used by the matcher to check for correspondences.
    #[serde(skip_serializing)]
    pub _trigger_sequences: Vec<Vec<TriggerEntry>>,
}

#[derive(Debug, Serialize, Clone)]
pub enum MatchContentType {
    Text(TextContent),
    Image(ImageContent),
}

#[derive(Debug, Serialize, Clone)]
pub struct TextContent {
    pub replace: String,
    pub vars: Vec<MatchVariable>,

    #[serde(skip_serializing)]
    pub _has_vars: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageContent {
    pub path: PathBuf,
}

impl<'de> serde::Deserialize<'de> for Match {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let auto_match = AutoMatch::deserialize(deserializer)?;
        Ok(Match::from(&auto_match))
    }
}

impl<'a> From<&'a AutoMatch> for Match {
    fn from(other: &'a AutoMatch) -> Self {
        lazy_static! {
            static ref VAR_REGEX: Regex = Regex::new("\\{\\{\\s*(\\w+)\\s*\\}\\}").unwrap();
        };

        let mut triggers = if !other.triggers.is_empty() {
            other.triggers.clone()
        } else if !other.trigger.is_empty() {
            vec![other.trigger.clone()]
        } else {
            panic!("Match does not have any trigger defined: {:?}", other)
        };

        // If propagate_case is true, we need to generate all the possible triggers
        // For example, specifying "hello" as a trigger, we need to have:
        // "hello", "Hello", "HELLO"
        if other.propagate_case {
            // List with first letter capitalized
            let first_capitalized: Vec<String> = triggers
                .iter()
                .map(|trigger| {
                    let capitalized = trigger.clone();
                    let mut v: Vec<char> = capitalized.chars().collect();

                    // Capitalize the first alphabetic letter
                    // See issue #244
                    let first_alphabetic = v.iter().position(|c| c.is_alphabetic()).unwrap_or(0);

                    v[first_alphabetic] = v[first_alphabetic].to_uppercase().nth(0).unwrap();
                    v.into_iter().collect()
                })
                .collect();

            let all_capitalized: Vec<String> = triggers
                .iter()
                .map(|trigger| trigger.to_uppercase())
                .collect();

            triggers.extend(first_capitalized);
            triggers.extend(all_capitalized);
        }

        let trigger_sequences = triggers
            .iter()
            .map(|trigger| {
                // Calculate the trigger sequence
                let mut trigger_sequence = Vec::new();
                let trigger_chars: Vec<char> = trigger.chars().collect();
                trigger_sequence.extend(trigger_chars.into_iter().map(|c| TriggerEntry::Char(c)));
                if other.word {
                    // If it's a word match, end with a word separator
                    trigger_sequence.push(TriggerEntry::WordSeparator);
                }

                trigger_sequence
            })
            .collect();

        let has_vars = VAR_REGEX.is_match("");

        let content = TextContent {
            replace: String::from(""),
            vars: other.vars.clone(),
            _has_vars: has_vars,
        };

        let content_type = MatchContentType::Text(content);

        Self {
            triggers,
            content: content_type,
            word: other.word,
            passive_only: other.passive_only,
            _trigger_sequences: trigger_sequences,
            propagate_case: other.propagate_case,
            force_clipboard: other.force_clipboard,
        }
    }
}

/// Used to deserialize the Match struct before applying some custom elaboration.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AutoMatch {
    #[serde(default = "default_trigger")]
    pub trigger: String,

    #[serde(default = "default_triggers")]
    pub triggers: Vec<String>,

    #[serde(default = "default_replace")]
    pub replace: Option<String>,

    #[serde(default = "default_image_path")]
    pub image_path: Option<String>,

    #[serde(default = "default_vars")]
    pub vars: Vec<MatchVariable>,

    #[serde(default = "default_word")]
    pub word: bool,

    #[serde(default = "default_passive_only")]
    pub passive_only: bool,

    #[serde(default = "default_propagate_case")]
    pub propagate_case: bool,

    #[serde(default = "default_force_clipboard")]
    pub force_clipboard: bool,
}

fn default_trigger() -> String {
    "".to_owned()
}
fn default_triggers() -> Vec<String> {
    Vec::new()
}
fn default_vars() -> Vec<MatchVariable> {
    Vec::new()
}
fn default_word() -> bool {
    false
}
fn default_passive_only() -> bool {
    false
}
fn default_replace() -> Option<String> {
    None
}
fn default_image_path() -> Option<String> {
    None
}
fn default_propagate_case() -> bool {
    false
}
fn default_force_clipboard() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchVariable {
    pub name: String,

    #[serde(rename = "type")]
    pub var_type: String,

    #[serde(default = "default_params")]
    pub params: Mapping,
}

fn default_params() -> Mapping {
    Mapping::new()
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TriggerEntry {
    Char(char),
    WordSeparator,
}

pub trait MatchReceiver {
    fn on_match(&self, m: &Match, trailing_separator: Option<char>, trigger_offset: usize);
    fn on_enable_update(&self, status: bool);
    fn on_passive(&self);
}

pub trait Matcher: KeyEventReceiver {
    fn handle_char(&self, c: &str);
    fn handle_modifier(&self, m: KeyModifier);
    fn handle_other(&self);
}

impl<M: Matcher> KeyEventReceiver for M {
    fn on_key_event(&self, e: KeyEvent) {
        match e {
            KeyEvent::Char(c) => {
                self.handle_char(&c);
            }
            KeyEvent::Modifier(m) => {
                self.handle_modifier(m);
            }
            KeyEvent::Other => {
                self.handle_other();
            }
        }
    }
}
