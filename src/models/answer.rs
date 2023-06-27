use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};
use uuid::Uuid;

use crate::schema::*;

use super::{error::{db_map_err, DbError}, IdentifiableWithUuid};

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = answers)]
pub struct Answer {
    id: String,
    pub label: String,
    pub correct: bool,
    question: String
}

impl IdentifiableWithUuid for Answer {
    fn id(&self) -> Uuid {
        Uuid::parse_str(&self.id)
            .expect("Corrupted Uuid! Cannot proceed.")
    }
}

impl Answer {
    pub fn new(label: &str, correct: bool, question: Uuid) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(),
            label: label.to_string(),
            correct,
            question: question.to_string()
        }
    }

    pub fn insert(self, conn: &mut SqliteConnection) -> Result<(), DbError> {
        diesel::insert_into(answers::table)
            .values(self)
            .execute(conn)
            .map_err(db_map_err)?;

        Ok(())
    }

    pub fn select_answers_for_question(question: Uuid, conn: &mut SqliteConnection)
    -> Result<Vec<Answer>, DbError> {
        answers::table
            .filter(answers::question.eq(question.to_string()))
            .load(conn)
            .map_err(db_map_err)
    }

    pub fn question_id(&self) -> Uuid {
        Uuid::parse_str(&self.question)
            .expect("Corrupted Uuid! Cannot proceed.")
    }
}
