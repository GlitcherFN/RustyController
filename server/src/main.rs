use std::sync::Arc;

use log::{error, info};
use tokio::sync::{Mutex, watch};

use graphql::graphql_api;
use ps_move::models::LedEffect;

use crate::ps_move::controller::PsMoveController;

mod graphql;
mod ps_move;
mod spawn_tasks;
mod tasks;

#[derive(Clone)]
pub enum LedEffectChange {
    All {
        effect: LedEffect,
    },
    Single {
        effect: LedEffect,
        bt_address: String,
    },
    Multiple {
        effect: LedEffect,
        bt_addresses: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, rx) = watch::channel(LedEffectChange::All {
        effect: LedEffect::Off,
    });
    let controllers = Arc::new(Mutex::new(Vec::<Box<PsMoveController>>::new()));

    spawn_tasks::run_move(rx, &controllers).await;
    match graphql_api::start(Arc::new(tx), controllers).await {
        Ok(_) => {}
        Err(err) => {
            error!("Couldn't start GraphQL! {}", err)
        }
    };

    info!("Shutting down...");
}
