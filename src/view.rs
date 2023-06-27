use std::fmt::Display;

use console::style;
use uuid::Uuid;

use crate::{error::CliError, models::{question::Question, answer::Answer}, interact::{choose_item, choose_items}};

use super::interact::demand_string;

pub fn input_question(topic: Option<String>) -> Result<Question, CliError> {
    let topic = topic.map(|t| Ok(t));
    let topic = topic.unwrap_or_else(|| demand_string(None, "Topic of the question"))?;

    let label = demand_string(None, "Question contents")?;

    Ok(Question::new(&label, &topic))
}

pub fn input_answers(question: Uuid) -> Result<Vec<Answer>, CliError> {
    let choices = AnswerChoices::all();

    let mut answers = Vec::with_capacity(2);

    loop {
        let correct;

        let i = choose_item(&choices, "Add answers to given question")?;

        match choices[i] {
            AnswerChoices::AddCorrectAnswer => 
                correct = true,
            AnswerChoices::AddIncorrectAnswer => 
                correct = false,
            AnswerChoices::Finish => {
                if answers.len() > 1 {
                    break
                }
                else {
                    let msg = style(format!("{} {}", "✘", "Not enough answers, need at least two!")).red().bold();
                    println!("{}", msg);
                    continue
                }
            },
        }

        let label = demand_string(None, "Answer contents")?;

        answers.push(Answer::new(&label, correct, question));
    }

    Ok(answers)
}

pub fn ask_question(question: &Question, answers: &Vec<Answer>) -> Result<f64, CliError> {
    let choices = choose_items(&answers, &question.label)?;

    let mut correct_choices = 0;

    for i in choices {
        if answers[i].correct {
            correct_choices += 1;
        }
        else {
            return Ok(0.)
        }
    }

    let correct_answers = answers.iter().filter(|ans| ans.correct).count();

    Ok((correct_choices as f64)/(correct_answers as f64))
}

pub fn reveal_answer(question: &Question, answers: &Vec<Answer>) {
    println!("● {}", style(&question.label).bold().blue());

    for answer in answers {
        let styled;

        if answer.correct {
            styled = style(format!("{} {}","✔ " , &answer.label)).green();
        }
        else {
            styled = style(format!("{} {}","✘ " , &answer.label)).red();
        }

        println!("{}", styled);
    }
}

pub fn print_score(score: f64, i: usize) {
    let score_info = format!("Total score: {:.2}%", (score*100./(i+1) as f64));
    let styled_score = style(score_info.to_string()).bold().blue();

    println!("{}", styled_score);
}

enum AnswerChoices {
    AddCorrectAnswer,
    AddIncorrectAnswer,
    Finish
}

impl AnswerChoices {
    pub fn all() -> [AnswerChoices; 3] {
        [
            Self::AddCorrectAnswer,
            Self::AddIncorrectAnswer,
            Self::Finish
        ]
    }
}

impl Display for AnswerChoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddCorrectAnswer => write!(f, "Add correct answer"),
            Self::AddIncorrectAnswer => write!(f, "Add incorrect answer"),
            Self::Finish => write!(f, "Finish adding answers"),
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
