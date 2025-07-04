use std::sync::LazyLock;
use std::time::Duration;
use rbatis::RBatis;
use rbdc_pg::driver::PgDriver;

use crate::util::app_config::APP_CONFIG;

pub static RB: LazyLock<RBatis> = LazyLock::new(|| RBatis::new());

pub async fn init_rbatis() {
    println!("{}", APP_CONFIG.database.url.as_str());
    RB.link(PgDriver{}, APP_CONFIG.database.url.as_str()).await.unwrap();
    let pool = RB.get_pool().expect("pool not init!");
    //max connections
    pool.set_max_open_conns(APP_CONFIG.database.pool_size)
        .await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(
        APP_CONFIG.database.timeout_seconds,
    ))).await;
    
    tracing::info!("[eladmin_rs] rbatis pool init success! pool state = {:?}", pool.state().await);
}