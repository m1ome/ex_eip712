use eip_712::{EIP712, hash_structured_data};
use serde_json::from_str;
use ethsign::SecretKey;
use rustc_hex::{FromHex, ToHex};
use anyhow::{anyhow, Result};
use rustler::{Encoder, Env, NifResult, Term};
use web3_hash_utils::{hash_message};

mod atoms {
    rustler::atoms! {
      ok,
      error
    }
}

#[rustler::nif]
pub fn sign<'a>(env: Env<'a>, message: String, secret: String) -> NifResult<Term<'a>> {
    let s :Vec<u8> = match secret.from_hex() {
        Ok(secret) => secret,
        Err(error) => return Ok((atoms::error(), error.to_string()).encode(env))
    };

    return match sign_typed_data(&message, &s) {
        Ok(signature) => {
            Ok((atoms::ok(), format!("0x{}", signature)).encode(env))
        },
        Err(error) => Ok((atoms::error(), error.to_string()).encode(env))
    }
}

#[rustler::nif]
pub fn sign_message<'a>(env: Env<'a>, message: String, secret: String) -> NifResult<Term<'a>> {
    let s :Vec<u8> = match secret.from_hex() {
        Ok(secret) => secret,
        Err(error) => return Ok((atoms::error(), error.to_string()).encode(env))
    };

    return match sign_plain_message(&message, &s) {
        Ok(signature) => {
            Ok((atoms::ok(), format!("0x{}", signature)).encode(env))
        },
        Err(error) => Ok((atoms::error(), error.to_string()).encode(env))
    }
}

rustler::init!("Elixir.EIP712", [sign, sign_message]);

fn sign_typed_data(message: &str, secret: &[u8]) -> Result<String>{
    // Hashing message as EIP-712
    let typed_data = from_str::<EIP712>(message)?;
    let hash = hash_structured_data(typed_data.clone())
        .map_err(|err| anyhow!("can't calculate hash {}", err))?;

    // Signing message
    let secret = SecretKey::from_raw(secret)?;
    let signature = secret.sign(hash.as_bytes())?;

    let mut sig = [0u8; 65];
    sig[0..32].copy_from_slice(signature.r.as_ref());
    sig[32..64].copy_from_slice(signature.s.as_ref());
    sig[64] = signature.v + 27;

    return Ok(sig.to_hex());
}

fn sign_plain_message(message: &str, secret: &[u8]) -> Result<String> {
    let hash = hash_message(message);

    let secret = SecretKey::from_raw(secret)?;
    let signature = secret.sign(hash.as_bytes())?;

    let mut sig = [0u8; 65];
    sig[0..32].copy_from_slice(signature.r.as_ref());
    sig[32..64].copy_from_slice(signature.s.as_ref());
    sig[64] = signature.v + 27;

    return Ok(sig.to_hex());
}

#[test]
fn it_signs_message() {
    let string = "100:200:300";
    
    let secret :Vec<u8>  = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        .from_hex()
        .unwrap();
        
    let res = sign_message(string, &secret).unwrap();
    assert_eq!(res, "7b03070fcceedd12d2e4c7ac8e12c6567dfc41b1b416a4cd2a3fb6ca390666d32f28f202d6bc241e7af4a7b5ccde03061de8c4a944e32e5a439157c7d4c074841b")
}

#[test]
fn it_signs_v4() {
    let string = r#"
    {"types":{"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"Nft":[{"name":"tokenHash","type":"string"},{"name":"price","type":"uint256"},{"name":"receivers","type":"address[]"},{"name":"percents","type":"uint256[]"}]},"primaryType":"Nft","domain":{"name":"Gallery","version":"4","chainId":"0x7A69","verifyingContract":"0x5FbDB2315678afecb367f032d93F642f64180aa3"},"message":{"tokenHash":"1e59237e-f0f6-48b5-b384-17270eab0abb","price":"0x6F05B59D3B20000","receivers":["0x70997970C51812dc3A010C7d01b50e0d17dc79C8","0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"],"percents":["0x7D0","0x3E8"]}}
    "#;

    let secret :Vec<u8>  = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        .from_hex()
        .unwrap();

    let res = sign_typed_data(string, &secret).unwrap();
    assert_eq!(res, "678ae8dc200da9410bec6e218ecce1b3795ce21ccab1438757abad08e282ec826a8b1b04fe55b24c9763b62015deeb079e9695a979d187d0ae73942dab3905e41c");
}

#[test]
fn it_signs_v3() {
    let string = r#"
    {"types":{"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"Person":[{"name":"name","type":"string"},{"name":"wallet","type":"address"}],"Mail":[{"name":"from","type":"Person"},{"name":"to","type":"Person"},{"name":"contents","type":"string"}]},"primaryType":"Mail","domain":{"name":"Ether Mail","version":"1","chainId":"0x4","verifyingContract":"0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"},"message":{"from":{"name":"Cow","wallet":"0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"},"to":{"name":"Bob","wallet":"0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"},"contents":"Hello, Bob!"}}
    "#;

    let secret :Vec<u8>  = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        .from_hex()
        .unwrap();

    let res = sign_typed_data(string, &secret).unwrap();
    assert_eq!(res, "d853c5daec0c31492ae3f9f528105b4c29724b9b386231489d644b773c6d3afe48e4b120efc4c8a5e69bcb60cd4975ff1d3f196b18bf8d1a7df25db9c82cb0d01c");
}
