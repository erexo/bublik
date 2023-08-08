use anyhow::{Context, Result};
use sqlx::{migrate::MigrateDatabase, query, Pool, QueryBuilder, Row, Sqlite, SqlitePool};
use std::{
    fs::File,
    io::{BufReader, ErrorKind},
};
use tracing::{debug, info, trace};

use crate::models::{bank::Bank, card::Card, user::User};

const DB_URL: &str = "sqlite://bublik.db";

pub async fn prepare_database() -> Result<Pool<Sqlite>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating databse {}", DB_URL);
        Sqlite::create_database(DB_URL)
            .await
            .context("created database")?;
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    load_users(&db).await?;
    load_cards(&db).await?;
    load_banks(&db).await?;
    Ok(db)
}

async fn load_users(db: &Pool<Sqlite>) -> Result<()> {
    query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, firstName TEXT NOT NULL, lastName TEXT NOT NULL, email TEXT NOT NULL UNIQUE, phone TEXT NOT NULL, birthday TEXT NOT NULL, userType INTEGER NOT NULL);")
        .execute(db).await.context("create users")?;
    if query("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?
        .get::<i32, _>(0)
        == 0
    {
        match File::open("data/users.json") {
            Ok(file) => {
                trace!("Adding users data");
                let reader = BufReader::new(file);
                let users: Vec<User> = serde_json::from_reader(reader)?;
                debug!("Loaded {} users", users.len());
                let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
                    "INSERT INTO users(id, firstName, lastName, email, phone, birthday, userType) ",
                );
                query.push_values(users, |mut query, user| {
                    query
                        .push_bind(user.id)
                        .push_bind(user.first_name)
                        .push_bind(user.last_name)
                        .push_bind(user.email)
                        .push_bind(user.phone)
                        .push_bind(user.birthday)
                        .push_bind(user.user_type as u32);
                });
                let result = query.build().execute(db).await.context("users insert")?;
                info!("Added {} users", result.rows_affected());
            }
            Err(err) if err.kind() != ErrorKind::NotFound => return Err(err.into()),
            _ => {}
        }
    }
    Ok(())
}

async fn load_cards(db: &Pool<Sqlite>) -> Result<()> {
    query("CREATE TABLE IF NOT EXISTS cards (id INTEGER PRIMARY KEY NOT NULL, cardType INTEGER NOT NULL, number TEXT NOT NULL UNIQUE, expiration TEXT NOT NULL, owner TEXT NOT NULL);")
        .execute(db).await.context("create cards")?;
    if query("SELECT COUNT(*) FROM cards")
        .fetch_one(db)
        .await?
        .get::<i32, _>(0)
        == 0
    {
        match File::open("data/cards.json") {
            Ok(file) => {
                trace!("Adding cards data");
                let reader = BufReader::new(file);
                let cards: Vec<Card> = serde_json::from_reader(reader)?;
                debug!("Loaded {} cards", cards.len());
                let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
                    "INSERT INTO cards(id, cardType, number, expiration, owner) ",
                );
                query.push_values(cards, |mut query, card| {
                    query
                        .push_bind(card.id)
                        .push_bind(card.card_type as u32)
                        .push_bind(card.number)
                        .push_bind(card.expiration)
                        .push_bind(card.owner);
                });
                let result = query.build().execute(db).await.context("cards insert")?;
                info!("Added {} cards", result.rows_affected());
            }
            Err(err) if err.kind() != ErrorKind::NotFound => return Err(err.into()),
            _ => {}
        }
    }
    Ok(())
}

async fn load_banks(db: &Pool<Sqlite>) -> Result<()> {
    query("CREATE TABLE IF NOT EXISTS banks (id INTEGER PRIMARY KEY NOT NULL, country TEXT NOT NULL, city TEXT NOT NULL, zipcode TEXT NOT NULL, street TEXT NOT NULL, buildingNumber TEXT NOT NULL);")
        .execute(db).await.context("create banks")?;
    if query("SELECT COUNT(*) FROM banks")
        .fetch_one(db)
        .await?
        .get::<i32, _>(0)
        == 0
    {
        match File::open("data/banks.json") {
            Ok(file) => {
                trace!("Adding banks data");
                let reader = BufReader::new(file);
                let banks: Vec<Bank> = serde_json::from_reader(reader)?;
                debug!("Loaded {} banks", banks.len());
                let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
                    "INSERT INTO banks(id, country, city, zipcode, street, buildingNumber) ",
                );
                query.push_values(banks, |mut query, bank| {
                    query
                        .push_bind(bank.id)
                        .push_bind(bank.country)
                        .push_bind(bank.city)
                        .push_bind(bank.zipcode)
                        .push_bind(bank.street)
                        .push_bind(bank.building_number);
                });
                let result = query.build().execute(db).await.context("banks insert")?;
                info!("Added {} banks", result.rows_affected());
            }
            Err(err) if err.kind() != ErrorKind::NotFound => return Err(err.into()),
            _ => {}
        }
    }
    Ok(())
}
