#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Client, Proxy};

#[tauri::command]
async fn check_proxy(
    ip: &str,
    port: u16,
    username: Option<&str>,
    password: Option<&str>,
    url: &str,
) -> Result<String, String> {
    let proxy = get_reqwest_proxy(ip, port, username, password).expect("Failed to create proxy");
    let client = get_reqwest_client(proxy).expect("Failed to create client");

    let response = match client.get(url).send().await {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e);
            return Err("Request failed".into());
        }
    };

    let status = response.status();
    let status_code = status.as_u16();

    let message = match status.is_success() || status.is_informational() {
        true => "working",
        false => "failed",
    };

    Ok(format!("{status_code} ({message})"))
}

fn get_reqwest_proxy(
    ip: &str,
    port: u16,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<Proxy, String> {
    let proxy_url = format!("http://{ip}:{port}");
    let mut proxy = Proxy::all(proxy_url).unwrap();

    if username.is_some() && password.is_some() {
        proxy = proxy.basic_auth(username.unwrap(), password.unwrap());
    }

    Ok(proxy)
}

fn get_reqwest_client(proxy: Proxy) -> Result<Client, String> {
    let client = Client::builder().proxy(proxy).build().unwrap();
    Ok(client)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
