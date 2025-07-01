// src/api.rs
use crate::crypto::{double_sha256, generate_key_pair, sign, verify, calculate_merkle_root};
use crate::models::{Node, Transaction, Block, BlockHeader, UTXO, Output};
use reqwest::{Client, Error};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: f64 = 1.0;
const SEED_NODE: &str = "192.168.1.117:8133";

pub struct NodeClient {
    client: Client,
    nodes: Vec<Node>,
}

impl NodeClient {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::new();
        let nodes = discover_nodes(&client).await?;
        Ok(Self { client, nodes })
    }

    pub async fn discover_nodes(client: &Client) -> Result<Vec<Node>, Error> {
        let url = format!("http://{}/discover/nodes", SEED_NODE);
        let response = client.get(&url).send().await?;
        let nodes = response.json().await?;
        Ok(nodes)
    }

    pub async fn broadcast_transaction(&self, tx: &Transaction) -> Result<bool, Error> {
        let mut responses = Vec::new();
        
        for node in &self.nodes {
            let url = format!("http://{}:{}/validate/tx", node.ipv4_address, node.port);
            let response = self.client.post(&url)
                .json(tx)
                .send()
                .await?
                .json::<Value>()
                .await?;
            
            responses.push(response);
        }

        // Find most common response (mode)
        let mut counts = HashMap::new();
        for response in &responses {
            *counts.entry(response).or_insert(0) += 1;
        }
        
        let mode = counts.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap();

        Ok(mode["valid"].as_bool().unwrap_or(false))
    }

    pub async fn send(
        &self,
        private_key: &str,
        amount: u64,
        address: &str,
        fee: u64,
    ) -> Result<bool, Error> {
        // Get UTXOs
        let (inputs, mempool_inputs) = self.get_utxos().await?;
        let mut all_inputs = [inputs, mempool_inputs].concat();

        // Select inputs
        let mut selected_inputs = Vec::new();
        let mut total = 0;
        
        for input in all_inputs.drain(..) {
            if total >= amount + fee {
                break;
            }
            total += input.amount;
            selected_inputs.push(input);
        }

        // Generate outputs
        let outputs = generate_outputs(selected_inputs, amount, address, fee)?;
        if outputs.is_empty() {
            return Ok(false);
        }

        // Create and sign transaction
        let mut tx = Transaction {
            version: VERSION as u32,
            inputs: selected_inputs,
            outputs,
            signature: None,
            txid: None,
            public_key: None,
        };

        let (signature, pubkey) = sign(private_key, &serde_json::to_string(&tx)?);
        tx.signature = Some(signature);
        tx.public_key = Some(pubkey);

        let txid = hex::encode(double_sha256(&serde_json::to_vec(&tx)?));
        tx.txid = Some(txid);

        // Broadcast
        self.broadcast_transaction(&tx).await
    }

    async fn get_utxos(&self) -> Result<(Vec<UTXO>, Vec<UTXO>), Error> {
        let mut responses = Vec::new();
        
        for node in &self.nodes {
            let utxos = self.client.get(&format!(
                "http://{}:{}/utxos/address/{}", 
                node.ipv4_address, node.port, address
            )).send().await?.json().await?;
            
            let mempool_utxos = self.client.get(&format!(
                "http://{}:{}/utxos_mempool/address/{}",
                node.ipv4_address, node.port, address
            )).send().await?.json().await?;
            
            responses.push((utxos, mempool_utxos));
        }

        // Find mode response
        let mut counts = HashMap::new();
        for response in &responses {
            *counts.entry(response).or_insert(0) += 1;
        }
        
        let mode = counts.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap();

        Ok(mode.clone())
    }
}

fn generate_outputs(
    inputs: Vec<UTXO>,
    amount: u64,
    address: &str,
    fee: u64,
) -> Result<Vec<Output>, &'static str> {
    let total: u64 = inputs.iter().map(|i| i.amount).sum();
    
    if total < amount + fee {
        return Ok(Vec::new());
    }

    let change = total - amount - fee;
    let mut outputs = vec![
        Output {
            amount,
            address: address.to_string(),
        }
    ];

    if change > 0 {
        outputs.push(Output {
            amount: change,
            address: own_address, // Would need to be passed in
        });
    }

    Ok(outputs)
}