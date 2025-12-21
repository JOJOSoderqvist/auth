create table if not exists "users" (
    "id" uuid not null primary key,
    "email" text not null unique,
    "username" text not null,
    "password_hash" text not null,
    "created_at" timestamp with time zone not null default current_timestamp,
    "updated_at" timestamp with time zone not null default current_timestamp
);
