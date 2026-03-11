use rain_orderbook_app_settings::yaml::{orderbook::OrderbookYaml, YamlParsable};
use reqwest::Client;
use serde_json::{json, Value};

fn load_settings() -> OrderbookYaml {
    let yaml = std::fs::read_to_string("settings.yaml").expect("failed to read settings.yaml");
    OrderbookYaml::new(vec![yaml], Default::default()).expect("failed to parse settings.yaml")
}

async fn test_rpc(client: &Client, url: &str, expected_chain_id: u32) -> Result<(), String> {
    let chain_id_resp = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        }))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("eth_chainId request failed: {e}"))?
        .json::<Value>()
        .await
        .map_err(|e| format!("eth_chainId response parse failed: {e}"))?;

    if let Some(error) = chain_id_resp.get("error") {
        return Err(format!("eth_chainId returned error: {error}"));
    }

    let result = chain_id_resp
        .get("result")
        .and_then(|r| r.as_str())
        .ok_or("eth_chainId missing result field")?;

    let actual_chain_id = u32::from_str_radix(result.trim_start_matches("0x"), 16)
        .map_err(|e| format!("failed to parse chain id: {e}"))?;

    if actual_chain_id != expected_chain_id {
        return Err(format!(
            "chain id mismatch: expected {expected_chain_id}, got {actual_chain_id}"
        ));
    }

    let eth_call_resp = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{"to": "0x0000000000000000000000000000000000000000", "data": "0x"}, "latest"],
            "id": 2
        }))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("eth_call request failed: {e}"))?
        .json::<Value>()
        .await
        .map_err(|e| format!("eth_call response parse failed: {e}"))?;

    if let Some(error) = eth_call_resp.get("error") {
        return Err(format!("eth_call returned error: {error}"));
    }

    if eth_call_resp.get("result").is_none() {
        return Err("eth_call missing result field".to_string());
    }

    Ok(())
}

#[tokio::test]
async fn all_rpcs_support_eth_call() {
    let settings = load_settings();
    let networks = settings.get_networks().expect("failed to get networks");
    let client = Client::new();
    let mut failures = Vec::new();

    for (name, network) in &networks {
        assert!(
            network.rpcs.len() >= 3,
            "{name} has only {} RPCs, expected at least 3",
            network.rpcs.len()
        );

        for rpc_url in &network.rpcs {
            if let Err(e) = test_rpc(&client, rpc_url.as_str(), network.chain_id).await {
                failures.push(format!("{name} {rpc_url}: {e}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "RPC failures:\n{}",
        failures.join("\n")
    );
}
