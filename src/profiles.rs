#![allow(dead_code)]

use crate::constants::DB_PATH;
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Profile {
    pub id: Option<i64>,
    pub user_id: String,
    pub username: String,
    pub name: String,
    pub wallet: String,
}

pub struct ProfileDatabase {
    pub conn: Connection,
}

impl ProfileDatabase {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(DB_PATH)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS profiles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                wallet TEXT NULL
            )",
            [],
        )?;

        Ok(ProfileDatabase { conn })
    }

    pub fn create(&self, profile: &Profile) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO profiles (user_id, username, name, wallet) VALUES (?1, ?2, ?3, ?4)",
            params![
                profile.user_id,
                profile.username,
                profile.name,
                profile.wallet
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get(&self, user_id: &str) -> Result<Option<Profile>> {
        let profile = self
            .conn
            .query_row(
                "SELECT id, user_id, username, name, wallet FROM profiles WHERE user_id = ?1",
                params![user_id],
                |row| {
                    Ok(Profile {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        username: row.get(2)?,
                        name: row.get(3)?,
                        wallet: row.get(4)?,
                    })
                },
            )
            .optional()?;

        Ok(profile)
    }

    pub fn update(&self, profile: &Profile) -> Result<()> {
        self.conn.execute(
            "UPDATE profiles SET username = ?1, name = ?2, wallet = ?3 WHERE user_id = ?4",
            params![
                profile.username,
                profile.name,
                profile.wallet,
                profile.user_id
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, user_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM profiles WHERE user_id = ?1", params![user_id])?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Profile>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, user_id, username, name, wallet FROM profiles")?;
        let profile_iter = stmt.query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                user_id: row.get(1)?,
                username: row.get(2)?,
                name: row.get(3)?,
                wallet: row.get(4)?,
            })
        })?;

        let mut profiles = Vec::new();
        for profile_result in profile_iter {
            profiles.push(profile_result?);
        }

        Ok(profiles)
    }

    pub fn upsert(&self, profile: &Profile) -> Result<i64> {
        let existing = self
            .conn
            .query_row(
                "SELECT id FROM profiles WHERE user_id = ?1",
                params![profile.user_id],
                |row| row.get::<_, i64>(0),
            )
            .optional()?;

        match existing {
            Some(existing_id) => {
                self.conn.execute(
                    "UPDATE profiles SET username = ?1, name = ?2, wallet = ?3 WHERE user_id = ?4",
                    params![
                        profile.username,
                        profile.name,
                        profile.wallet,
                        profile.user_id
                    ],
                )?;
                Ok(existing_id)
            }
            None => {
                self.conn.execute(
                    "INSERT INTO profiles (user_id, username, name, wallet) VALUES (?1, ?2, ?3, ?4)",
                    params![profile.user_id, profile.username, profile.name, profile.wallet]
                )?;
                Ok(self.conn.last_insert_rowid())
            }
        }
    }
}
