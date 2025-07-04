use std::sync::LazyLock;
use std::time::Duration;
use rbatis::RBatis;
use rbdc_pg::driver::PgDriver;

use crate::util::app_config::APP_CONFIG;

pub static RB: LazyLock<RBatis> = LazyLock::new(|| RBatis::new());

pub async fn init_rbatis() {
    let db_cfg = APP_CONFIG.database.clone();
    RB.link(PgDriver{}, &db_cfg.url).await.unwrap();
    let pool = RB.get_pool().expect("pool not init!");
    //max connections
    pool.set_max_open_conns(db_cfg.pool_size)
        .await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(
        db_cfg.timeout_seconds,
    ))).await;
    
    tracing::info!("[eladmin_rs] rbatis pool init success! pool state = {:?}", pool.state().await);
}