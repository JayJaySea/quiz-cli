// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Text,
        label -> Text,
        correct -> Bool,
        question -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Text,
        label -> Text,
        topic -> Text,
    }
}

diesel::joinable!(answers -> questions (question));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
