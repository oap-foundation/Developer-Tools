<?php

namespace OAP\EchoBot;

use OAP\OAEP\DidKey;
use OAP\OAEP\KeyPair;
use OAP\OAEP\SessionKey;
use OAP\OATP\RelayClient;
use OAP\OATP\BlindInbox;
use OAP\OATP\Sharding\Sharding;
use Exception;

class Bot
{
    private KeyPair $identityKey;
    private string $did;
    private RelayClient $relay;
    private array $config;

    public function __construct(array $config)
    {
        $this->config = $config;
        $this->loadIdentity();
        $this->relay = new RelayClient($config['relay_url']);
    }

    private function loadIdentity(): void
    {
        if (!file_exists($this->config['key_file'])) {
            throw new Exception("Key file not found: " . $this->config['key_file']);
        }

        $data = json_decode(file_get_contents($this->config['key_file']), true);
        if (!$data)
            throw new Exception("Invalid key file");

        // Assuming keys.json format from generate_did.php
        // 'secret_key' is hex encoded
        $secret = hex2bin($data['secret_key']);
        $this->identityKey = KeyPair::fromSecretKey($secret);
        $this->did = $data['did'];
    }

    private function log(string $message, string $level = 'INFO'): void
    {
        echo "[" . date('Y-m-d H:i:s') . "] [$level] $message\n";
    }

    public function start(): void
    {
        $this->log("Starting Echo Bot...");
        $this->log("Relay: {$this->config['relay_url']}");
        $this->log("Identity: {$this->did}");

        while (true) {
            try {
                $this->poll();
            } catch (Exception $e) {
                $this->log("Critical error in poll loop: " . $e->getMessage(), 'ERROR');
            }

            sleep($this->config['poll_interval'] ?? 5);
        }
    }

    private function poll(): void
    {
        // 1. Derive Inbox ID for current epoch
        // For the Echo Bot, we need a Session Key to derive the Blind Inbox.
        // In a real scenario, this Session Key comes from the OAEP Handshake.
        // However, for the *initial* contact (Handshake Request), there is no shared session key yet.
        // OATP usually implies we are checking an inbox derived from a shared secret.
        // BUT, for a public bot, how do we receive the first message?
        // The OAP spec (Layer 0) likely defines a way to send the first handshake message.
        // Usually, it's sent to an inbox derived from the recipient's public key or a well-known inbox.

        // For this Echo Bot implementation, let's assume we are checking a "Public Inbox" 
        // or we are simulating an established session for testing.
        // OR, we might need to implement the "Handshake Inbox" logic if defined.

        // Let's assume for now we use a static "Bot Secret" to derive our inbox, 
        // effectively acting as a "long-term" session for simplicity, 
        // or we just check an inbox derived from our own Identity Key (acting as a session key).

        // Using Identity Secret as Session Key for Inbox Derivation (Simplification)
        // We use the X25519 secret key derived from the Ed25519 identity key.
        $x25519 = $this->identityKey->getX25519KeyPair();
        $sessionKey = new SessionKey($x25519['secret']);
        $inbox = BlindInbox::deriveCurrent($sessionKey);

        // $this->log("Checking Inbox: " . $inbox->toHex(), 'DEBUG');

        try {
            $response = $this->relay->retrieveShards($inbox);

            if (empty($response->shards)) {
                // No messages
                return;
            }

            $this->log("Received " . count($response->shards) . " shards.");

            foreach ($response->shards as $shard) {
                try {
                    // 1. Decrypt JWE
                    // Try with Identity Key first (Handshake)
                    $x25519 = $this->identityKey->getX25519KeyPair();
                    $plaintext = null;

                    try {
                        $jwe = \OAP\OAEP\JweContainer::fromCompact($shard['payload']); // Assuming payload is the JWE string
                        $plaintext = $jwe->decrypt($x25519['secret']);
                    } catch (Exception $e) {
                        // Try with active sessions? 
                        // In a real bot, we'd look up the session based on kid or threadId if visible.
                        // For simplicity, we might iterate or assume Identity Key for now as we don't persist sessions well yet.
                        $this->log("Failed to decrypt shard {$shard['id']}: " . $e->getMessage(), 'WARNING');
                        continue;
                    }

                    if (!$plaintext)
                        continue;

                    $message = json_decode($plaintext, true);
                    if (!$message) {
                        $this->log("Failed to decode JSON payload", 'WARNING');
                        continue;
                    }

                    $type = $message['@type'] ?? 'unknown';
                    $from = $message['from'] ?? 'unknown';
                    $this->log("Processing message '$type' from $from");

                    // 2. Handle ConnectionRequest
                    if ($type === 'ConnectionRequest') {
                        $handshake = new Handshake($this->identityKey, $this->did);
                        $result = $handshake->handleRequest($message);

                        // Send Response
                        $this->send($result['response'], $from);
                        $this->log("Sent ConnectionResponse to $from");

                        // Create Session (In-Memory)
                        // In a real app, we'd derive keys here.
                        // $this->sessionManager->createSession($message['threadId'], $message['from']);
                    }
                    // 3. Handle Basic Message (Echo)
                    elseif (isset($message['text'])) {
                        $replyText = "You said: " . $message['text'];
                        $reply = [
                            '@type' => 'Message',
                            'text' => $replyText,
                            'threadId' => $message['threadId'] ?? uniqid(),
                            'created' => gmdate('Y-m-d\TH:i:s\Z')
                        ];

                        $this->send($reply, $from);
                        $this->log("Echoed back to $from");
                    }

                } catch (Exception $e) {
                    $this->log("Error processing shard: " . $e->getMessage(), 'ERROR');
                }
            }

        } catch (Exception $e) {
            // 404 or empty is fine
            if (str_contains($e->getMessage(), "404")) {
                return;
            }
            $this->log("Poll failed: " . $e->getMessage(), 'WARNING');
        }
    }

    private function send(array $payload, string $toDid): void
    {
        // 1. Encrypt Payload
        // We need to derive a shared secret using ECDH.
        // My X25519 SK + Peer X25519 PK.

        if (str_starts_with($toDid, 'did:key:')) {
            try {
                // Resolve Peer DID to get Ed25519 Public Key
                $peerEd25519Pk = \OAP\OAEP\DidKey::resolve(new \OAP\OAEP\Did($toDid));

                // Convert Peer Ed25519 PK to X25519 PK
                $peerX25519Pk = sodium_crypto_sign_ed25519_pk_to_curve25519($peerEd25519Pk);

                // Get My X25519 SK
                $myX25519 = $this->identityKey->getX25519KeyPair();

                // Perform ECDH
                $sharedSecret = sodium_crypto_scalarmult($myX25519['secret'], $peerX25519Pk);

                // Encrypt JWE using Shared Secret
                // We use a random kid and seq=0 for this stateless echo
                $jwe = \OAP\OAEP\JweContainer::encrypt(
                    json_encode($payload),
                    $sharedSecret,
                    uniqid(), // kid
                    0 // seq
                );

                // In a real scenario, we would now shard this JWE and upload it.
                // For Phase 4 Echo Bot, we'll just log the encrypted payload to demonstrate success.
                // $shards = Sharding::encode($jwe->toCompact());
                // $this->relay->upload($shards);
                $this->log("[MOCK UPLOAD] Encrypted JWE for $toDid");

            } catch (Exception $e) {
                $this->log("Failed to encrypt for $toDid: " . $e->getMessage(), 'ERROR');
            }
        } else {
            $this->log("[MOCK SEND] To: $toDid (Encryption not supported for this DID type)", 'WARNING');
        }
    }
}
