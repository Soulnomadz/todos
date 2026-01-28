create table if not exists public.todos (
	id bigserial primary key,
	text varchar(255) not null,
	completed boolean not null default false,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
