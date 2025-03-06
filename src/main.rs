mod config;
mod database;
mod api;
use tokio_postgres::{connect, Error};
use std::collections::HashSet;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (apikey, db_url) = config::config::config();
    
    loop {
        println!("\n=== Starting validation cycle ===");
        let start_time = std::time::SystemTime::now();
        
        match run_validation_cycle(&apikey, &db_url).await {
            Ok(_) => println!("Validation cycle completed successfully"),
            Err(e) => eprintln!("Error in validation cycle: {}", e),
        }
        
        if let Ok(elapsed) = start_time.elapsed() {
            println!("Cycle completed in {:.2} seconds", elapsed.as_secs_f64());
        }
        
        println!("Sleeping for 60 seconds before next cycle...");
        sleep(Duration::from_secs(60)).await;
    }
}

async fn run_validation_cycle(apikey: &str, db_url: &str) -> Result<(), Error> {
    let (client, connection) = connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db_ips = database::database::select_all_from_db(&client).await?;
    println!("Found {} IPs in database", db_ips.len());
    
    let mut all_proxies = Vec::new();
    let mut page = 1;
    while page <= 3 { 
        match api::api::get_proxies(&apikey, page).await {
            Ok(proxy_response) => {
                println!("Page {}: Found {} proxies", page, proxy_response.results.len());
                all_proxies.extend(proxy_response.results);
                page += 1;
            },
            Err(e) => {
                eprintln!("Error fetching proxies: {}", e);
                break;
            }
        }
    }
    
    let api_ip_set: HashSet<String> = all_proxies.iter()
        .map(|proxy| proxy.proxy_address.clone())
        .collect();
        
    let db_ip_set: HashSet<String> = db_ips.iter().cloned().collect();

    let mut deleted = 0;
    for db_ip in &db_ips {
        if !api_ip_set.contains(db_ip) {
            match database::database::delete_ip(&client, db_ip).await {
                Ok(_) => {
                    println!("Deleted IP: {}", db_ip);
                    deleted += 1;
                },
                Err(e) => eprintln!("Error deleting IP {}: {}", db_ip, e),
            }
        }
    }
    
    let mut updated = 0;
    let mut inserted = 0;
    
    for proxy in &all_proxies {
        let ip = &proxy.proxy_address;
        
        if db_ip_set.contains(ip) {
            match database::database::update_ip(&client, proxy).await {
                Ok(_) => {
                    updated += 1;
                },
                Err(e) => eprintln!("Error updating IP {}: {}", ip, e),
            }
        } else {
            match database::database::insert_ip(&client, proxy).await {
                Ok(_) => {
                    println!("Inserted IP: {}", ip);
                    inserted += 1;
                },
                Err(e) => eprintln!("Error inserting IP {}: {}", ip, e),
            }
        }
    }
    
    println!("Summary: {} deleted, {} updated, {} inserted", deleted, updated, inserted);
    
    Ok(())
}