use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use oap::oaep::messages::{ConnectionRequest, ConnectionResponse, HandshakeTranscript, TranscriptHeader, TranscriptParty};
use oap::oaep::keys::{X25519Secret, SessionKey, X25519Public};
use oap::oaep::types::{CipherSuite, Nonce, Timestamp};
use oap::oaep::did::Did;
use oap::oatp::container::{JweContainer, decrypt_padded}; // Assuming export
use hex; 

#[derive(Clone, Debug)]
pub struct HandshakeContext {
    pub request_id: String,
    pub initiator_did: Did,
    pub responder_did: Option<Did>,
    pub initiator_nonce: Option<Nonce>,
    pub responder_nonce: Option<Nonce>,
    pub initiator_ephemeral: Option<String>, // Multibase
    pub responder_ephemeral: Option<String>,
    pub created: Option<Timestamp>,
    pub suite: Option<CipherSuite>,
    pub transcript_hash: Option<Vec<u8>>,
}

pub struct XRayState {
    pub contexts: Mutex<HashMap<String, HandshakeContext>>,
    pub sessions: Mutex<HashMap<String, (SessionKey, SessionKey)>>,
}

impl XRayState {
    pub fn new() -> Self {
        Self {
            contexts: Mutex::new(HashMap::new()),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn process_packet(&self, body: &str, user_secrets: &[String]) -> Option<String> {
        // 1. Try to parse as Handshake Message
        if let Ok(req) = serde_json::from_str::<ConnectionRequest>(body) {
            self.handle_request(req);
            return Some("Captured ConnectionRequest".to_string());
        }
        if let Ok(res) = serde_json::from_str::<ConnectionResponse>(body) {
            self.handle_response(res, user_secrets);
            return Some("Captured ConnectionResponse".to_string());
        }

        // 2. Try to decrypt as OATP Container
        self.try_decrypt_all(body)
    }

    fn handle_request(&self, req: ConnectionRequest) {
        let mut ctxs = self.contexts.lock().unwrap();
        let ctx = HandshakeContext {
            request_id: req.id.clone(),
            initiator_did: req.from,
            responder_did: None, // Will fill on response
            initiator_nonce: Some(req.body.nonce),
            responder_nonce: None,
            initiator_ephemeral: Some(req.body.key_exchange.public_key),
            responder_ephemeral: None,
            created: Some(req.created),
            suite: req.body.key_exchange.supported_suites.first().cloned(),
            transcript_hash: None,
        };
        ctxs.insert(req.id, ctx);
    }

    fn handle_response(&self, res: ConnectionResponse, user_secrets: &[String]) {
        let mut ctxs = self.contexts.lock().unwrap();
        // Link via reply_to -> request_id
        if let Some(ctx) = ctxs.get_mut(&res.reply_to) {
            ctx.responder_did = Some(res.from);
            ctx.responder_nonce = Some(res.body.nonce);
            ctx.responder_ephemeral = Some(res.body.key_exchange.public_key);
            ctx.suite = Some(res.body.key_exchange.negotiated_suite);

            if let (Some(init_key), Some(resp_key)) = (ctx.initiator_ephemeral.clone(), ctx.responder_ephemeral.clone()) {
                
                // Use hash from proof
                let hash_hex = res.proof.transcript_hash;
                let hash_bytes = match hex::decode(&hash_hex) {
                    Ok(b) => b,
                    Err(_) => return, // Invalid hex
                };

                // STORE HASH in Context (for future encryption)
                ctx.transcript_hash = Some(hash_bytes.clone());

                // Try to derive keys
                for secret_str in user_secrets {
                     if let Some(secret) = Self::parse_secret(secret_str) {
                         let my_pub = secret.public_key().to_multibase();
                         
                         let peer_key_str = if my_pub == init_key {
                             Some(&resp_key)
                         } else if my_pub == resp_key {
                             Some(&init_key)
                         } else {
                             None
                         };

                         if let Some(peer_str) = peer_key_str {
                             if let Ok(peer_pub) = X25519Public::from_multibase(peer_str) {
                                  let shared = secret.diffie_hellman(&peer_pub);
                                  let info = b"OAEP-v1-Session-Keys";
                                  
                                  let (k1, k2) = SessionKey::derive(&shared, &hash_bytes, info);
                                  
                                  let mut sessions = self.sessions.lock().unwrap();
                                  sessions.insert(ctx.request_id.clone(), (k1, k2));
                                  println!("Success: Derived session keys for {}", ctx.request_id);
                             }
                         }
                     }
                }
            }
        }
    }

    pub fn encrypt_packet(&self, request_id: &str, plaintext: &str) -> Option<String> {
        // 1. Get Context to find KID (Transcript Hash)
        let binding = self.contexts.lock().unwrap();
        let ctx = binding.get(request_id)?;
        let hash = ctx.transcript_hash.as_ref()?;
        
        // KID is hex of first 16 bytes of hash
        let kid = hex::encode(&hash[0..16]);

        // 2. Get Keys
        let sessions = self.sessions.lock().unwrap();
        let (k1, _) = sessions.get(request_id)?;
        
        // Assumption: Replaying as Initiator (Client -> Server) => Use k1
        // To support Response replay/spoofing, we'd need a flag.
        
        let seq = 999999; // Mock Sequence for Replay

        use oap::oatp::container::encrypt_padded;
        
        match encrypt_padded(plaintext.as_bytes(), k1.as_bytes(), &kid, seq, 1024) {
            Ok(container) => serde_json::to_string(&container).ok(),
            Err(e) => {
                println!("Encryption failed: {}", e);
                None
            }
        }
    }

    fn parse_secret(s: &str) -> Option<X25519Secret> {
        // Try Multibase
        if let Ok((_, bytes)) = multibase::decode(s) {
            if bytes.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                return Some(X25519Secret::from_bytes(arr));
            }
        }
        // Try Hex
        if let Ok(bytes) = hex::decode(s) {
            if bytes.len() == 32 {
                 let mut arr = [0u8; 32];
                 arr.copy_from_slice(&bytes);
                 return Some(X25519Secret::from_bytes(arr));
            }
        }
        None
    }

    fn try_decrypt_all(&self, body: &str) -> Option<String> {
        // Try parsing as JweContainer
        if let Ok(container) = serde_json::from_str::<JweContainer>(body) {
             let sessions = self.sessions.lock().unwrap();
             // Iterate all sessions and both keys
             for (k1, k2) in sessions.values() {
                  if let Ok(plaintext) = decrypt_padded(&container, k1.as_bytes()) {
                      return String::from_utf8(plaintext).ok();
                  }
                  if let Ok(plaintext) = decrypt_padded(&container, k2.as_bytes()) {
                      return String::from_utf8(plaintext).ok();
                  }
             }
        }
        None
    }
}
