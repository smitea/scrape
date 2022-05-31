use std::ops::Add;

use hex_literal::hex;
use web3::{
    contract::{tokens::Detokenize, Contract, Options},
    ethabi::Token,
    types::{Address, Bytes, CallRequest, U256},
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);
    let contract_addr: Address = hex!("465a4A8DAA955B837957230385AC4A9997aa9d27").into();

    let my_account: Address = hex!("72d67E96950B7E66AF81AFE1C32307128658d98e").into();

    let contract = Contract::from_json(
        web3.eth(),
        contract_addr,
        include_bytes!("../abi/AuthToken.json"),
    )?;

    let mut addrs: Vec<Token> = vec![];
    let mut tokens: Vec<Token> = vec![];
    let mut uris: Vec<Token> = vec![];

    let mut token_id: usize = 0;
    for account in web3.eth().accounts().await? {
        let bytes = account.as_bytes();
        addrs.push(Token::Address(Address::from_slice(bytes)));
        tokens.push(Token::Uint(token_id.into()));
        uris.push(Token::String(token_id.to_string()));
        token_id += 1;
    }

    let params = vec![
        Token::Array(addrs.clone()),
        Token::Array(tokens.clone()),
        Token::Array(uris.clone()),
    ];

    let bytes = contract
        .abi()
        .function("batchMintWithURI")?
        .encode_input(&params)?;

    let gas_price = web3.eth().gas_price().await?;

    let gaslimit = web3
        .eth()
        .estimate_gas(
            CallRequest::builder()
                .gas_price(gas_price)
                .data(Bytes::from(bytes))
                .from(my_account)
                .to(contract_addr)
                .build(),
            None,
        )
        .await?;

    let options = Options::with(move |a| {
        a.gas = Some(gaslimit);
        a.gas_price = Some(gas_price);
    });

    println!("send call: batchMintWithURI");

    let tx = contract
        .call(
            "batchMintWithURI",
            (addrs, tokens, uris),
            my_account,
            options,
        )
        .await?;

    println!("got tx: {:?}", tx);
    return Ok(());
}
