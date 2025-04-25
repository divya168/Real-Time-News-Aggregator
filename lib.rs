#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct News {
    pub id: u64,
    pub title: String,
    pub source: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum NewsBook {
    Article(u64),
}

const NEWS_COUNT: Symbol = symbol_short!("N_COUNT");

#[contract]
pub struct NewsAggregatorContract;

#[contractimpl]
impl NewsAggregatorContract {
    pub fn add_news(env: Env, title: String, source: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&NEWS_COUNT).unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();
        let news = News {
            id: count,
            title,
            source,
            timestamp,
        };

        env.storage().instance().set(&NewsBook::Article(count), &news);
        env.storage().instance().set(&NEWS_COUNT, &count);

        count
    }

    pub fn get_news(env: Env, id: u64) -> News {
        env.storage().instance().get(&NewsBook::Article(id)).unwrap()
    }

    pub fn total_articles(env: Env) -> u64 {
        env.storage().instance().get(&NEWS_COUNT).unwrap_or(0)
    }
}

