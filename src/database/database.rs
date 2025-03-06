use tokio_postgres::{Client, Error};
use crate::api::api::Proxy;

pub async fn select_all_from_db(client: &Client) -> Result<Vec<String>, Error> {
    let rows = client.query("SELECT ip FROM proxy", &[]).await?;

    let mut ipvec: Vec<String> = Vec::new();
    for row in rows {
        let ip: &str = row.get("ip");
        ipvec.push(ip.to_string());
    }
    Ok(ipvec)
}

pub async fn delete_ip(client: &Client, ip: &str) -> Result<u64, Error> {
    let rows_affected = client.execute("DELETE FROM proxy WHERE ip = $1", &[&ip]).await?;
    Ok(rows_affected)
}

pub async fn update_ip(client: &Client, proxy: &Proxy) -> Result<u64, Error> {
    let rows_affected = client.execute(
        "UPDATE proxy SET
            port = $2,
            username = $3,
            password = $4
        WHERE ip = $1",
        &[
            &proxy.proxy_address,
            &(proxy.port as i32),
            &proxy.username,
            &proxy.password,
        ],
    ).await?;
    Ok(rows_affected)
}

pub async fn insert_ip(client: &Client, proxy: &Proxy) -> Result<u64, Error> {
    let rows_affected = client.execute(
        "INSERT INTO proxy (ip, port, username, password)
        VALUES ($1, $2, $3, $4)",
        &[
            &proxy.proxy_address,
            &(proxy.port as i32),
            &proxy.username,
            &proxy.password,
        ],
    ).await?;
    Ok(rows_affected)
}