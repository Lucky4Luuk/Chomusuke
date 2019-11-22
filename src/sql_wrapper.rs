use std::sync::MutexGuard;
use serenity::{
    model::{
        user::CurrentUser,
        gateway::Ready,
        event::{
            ResumedEvent,
        },
        id::{GuildId, UserId},
        guild::{
            Member,
        },
        channel::*,
    },
    prelude::*,
    client::{
        Client,
        bridge::gateway::ShardManager,
    },
    framework::standard::{
        StandardFramework,
        Configuration,
        DispatchError,
        macros::group,
    },
};

use postgres::{
    Connection,
    TlsMode,
    params::ConnectParams,
    types::*,
    error::Error,
};

pub fn does_member_exist(conn: &Connection, userid: UserId) -> bool {
    let query_rows = conn.query(&format!("SELECT COUNT(1) FROM Users WHERE UserID = {};", userid), &[]).expect("DB query failed");
    let member_data_count: i64 = query_rows.get(0).get("count");

    member_data_count > 0
}

pub fn create_new_member(conn: &Connection, userid: UserId) -> Result<u64, Error> {
    conn.execute(&format!("INSERT INTO Users (UserID) VALUES ({UserID});", UserID = userid), &[])
}

pub fn update_value_raw(conn: &Connection, table: &String, var_name: &String, value: &String, condition: &String) -> Result<u64, Error> {
    conn.execute(&format!("UPDATE {table} SET {var_name} = {value} WHERE {condition};", table=table, var_name=var_name, value=value, condition=condition), &[])
}
