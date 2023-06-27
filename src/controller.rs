use diesel::SqliteConnection;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::interact::choose_item_fuzzy;
use crate::models::IdentifiableWithUuid;

use crate::models::answer::Answer;
use crate::models::error::DbError;
use crate::models::question::Question;
use crate::view::{ask_question, print_score, reveal_answer};
use crate::{error::CliError, view::{input_answers, input_question}};

pub struct Controller {
    pub conn: SqliteConnection,
}

impl Controller {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn add_question(mut self, topic: Option<String>) -> Result<&'static str, CliError> {
        let question = input_question(topic)?;
        let answers = input_answers(question.id())?;

        question.insert(&mut self.conn)
            .map_err(map_db_err)?;

        for answer in answers {
            answer.insert(&mut self.conn)
                .map_err(map_db_err)?;
        }

        Ok("Question added succesfully!")
    }

    pub fn delete_question(mut self) -> Result<&'static str, CliError> {
        let questions = Question::select_all(&mut self.conn)
            .map_err(map_db_err)?;

        let i = choose_item_fuzzy(&questions, "Choose question to delete")?;

        questions[i].delete(&mut self.conn)
                .map_err(map_db_err)?;

        Ok("Question deleted succesfully!")
    }

    pub fn start_quiz(mut self, topic: Option<String>) -> Result<&'static str, CliError> {
        let topic: Option<Result<String, CliError>> = topic.map(|t| Ok(t));
        let topic = topic.unwrap_or_else(|| {
            let topics = Question::select_topics(&mut self.conn)
                .map_err(map_db_err)?;

            let i = choose_item_fuzzy(&topics, "Choose topic for quiz to cover")?;

            Ok(topics[i].clone())
        })?;

        let mut questions = Question::select_question_for_topic(&topic, &mut self.conn)
            .map_err(map_db_err)?;

        let mut rng = thread_rng();
        questions.shuffle(&mut rng);

        let mut score = 0.;
        for (i, question) in questions.iter().enumerate() {
            let mut answers = 
                Answer::select_answers_for_question(question.id(), &mut self.conn)
                .map_err(map_db_err)?;

            answers.shuffle(&mut rng);
            score += ask_question(question, &answers)?;

            reveal_answer(question, &answers);
            print_score(score, i);
        }

        Ok("")
    }
}

fn map_db_err(e: DbError) -> CliError {
    match e {
        e => CliError::Generic(e.into())
    }
}
