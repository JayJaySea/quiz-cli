-- Your SQL goes here

create table answers (
	id			text not null primary key,
	label		text not null,
	correct		boolean not null,
	question 	text not null references questions(id) on delete cascade
)
