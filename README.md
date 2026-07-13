# Rust Newsletter API

A backend REST API built in Rust, following the book "Zero To Production in Rust" by Luca Palmieri. This project is my hands-on practice for backend Rust development, aimed at building real, production-style skills rather than just tutorial exercises.

## What it does

The API supports three core user stories for a simple email newsletter service:

- A visitor can subscribe to the newsletter
- The author can send an email update to all subscribers
- A subscriber can unsubscribe

## Tech stack

- Rust
- actix-web (web framework)
- tokio (async runtime)
- PostgreSQL
- sqlx (database access)

## Status

Work in progress. Currently building the subscription endpoint (`POST /subscriptions`).

## Why this project

I'm learning Rust with the goal of landing a fresher backend role, and eventually moving into blockchain development. This project is meant to demonstrate real backend fundamentals: API design, testing, error handling, and deployment, not just language syntax.
