use std::{ops::{Range, RangeInclusive}, fmt::Display};

use boolinator::Boolinator;
use console::style;
use dialoguer::{Input, theme::ColorfulTheme, Select, MultiSelect, FuzzySelect};

use crate::error::CliError;

pub fn choose_item<T: Display>
(items: &[T], message: &str) -> Result<usize, CliError> {

    (!items.is_empty()).ok_or(CliError::NoItemsToChooseFrom)?;

    let selected = Select::with_theme(&input_theme())
        .with_prompt(message)
        .items(items)
        .interact()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(selected)
}

pub fn choose_item_fuzzy<T: Display>
(items: &[T], message: &str) -> Result<usize, CliError> {

    (!items.is_empty()).ok_or(CliError::NoItemsToChooseFrom)?;

    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .items(items)
        .interact()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(selected)
}

pub fn choose_items<T: Display>
(items: &[T], message: &str) -> Result<Vec<usize>, CliError> {

    (!items.is_empty()).ok_or(CliError::NoItemsToChooseFrom)?;

    let selected = MultiSelect::with_theme(&input_theme())
        .with_prompt(message)
        .items(items)
        .interact()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(selected)
}

pub fn demand_string(initial: Option<&str>, prompt: &str) -> Result<String, CliError> {
    let initial = initial.unwrap_or("");

    let input: String = Input::with_theme(&input_theme())
        .with_prompt(prompt)
        .with_initial_text(initial)
        .interact_text()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(input)
}

pub fn ask_for_string(initial: Option<String>, prompt: &str) -> Result<Option<String>, CliError> {
    let mut parsed = initial.clone();
    let initial = initial.unwrap_or(String::new());

    Input::with_theme(&input_theme())
        .with_prompt(prompt)
        .default("".to_string())
        .with_initial_text(initial)
        .validate_with(|input: &String| -> Result<(), CliError> {
            parsed = (!input.trim().is_empty()).then(|| input.clone());
            Ok(())
        })
        .interact_text()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(parsed)
}

pub fn ask_for_int(initial: Option<i32>, prompt: &str, range: RangeInclusive<i32>) -> Result<Option<i32>, CliError> {
    let mut parsed = initial;
    let initial = initial
        .map(|num| num.to_string())
        .unwrap_or(String::new());

    Input::with_theme(&input_theme())
        .with_prompt(prompt)
        .with_initial_text(initial)
        .default("".to_string())
        .validate_with(|input: &String| -> Result<(), CliError> {
            parsed = parse_range(input, *range.start()..*range.end()+1)?;
            Ok(())
        })
        .interact_text()
        .map_err(|_| CliError::CannotReadUserInput)?;

    Ok(parsed)
}

fn input_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();
    theme.prompt_prefix = style("●".to_string());
    theme.prompt_suffix = style("\n>".to_string()).blue().bold();

    theme
}

pub fn parse_range(input: &String, range: Range<i32>) -> Result<Option<i32>, CliError> {
    if input.trim().is_empty() {
        Ok(None)
    }
    else {
        let converted = input.parse::<i32>()
            .map_err(|_| CliError::InvalidDigit)?;

        range
            .contains(&converted)
            .as_result((), CliError::InvalidDigit)?;
        
        Ok(Some(converted))
    }
}
