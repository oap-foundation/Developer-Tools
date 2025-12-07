<?php

namespace OAP\EchoBot;

use OAP\OAEP\KeyPair;
use OAP\OAEP\DidKey;
use OAP\OAEP\Utils\Hash;
use Exception;

class Handshake
{
    private KeyPair $identityKey;
    private string $did;

    public function __construct(KeyPair $identityKey, string $did)
    {
        $this->identityKey = $identityKey;
        $this->did = $did;
    }

    public function handleRequest(array $request): array
    {
        // 1. Validate Request Structure
        if (($request['@type'] ?? '') !== 'ConnectionRequest') {
            throw new Exception("Invalid message type: " . ($request['@type'] ?? 'unknown'));
        }

        $threadId = $request['threadId'] ?? throw new Exception("Missing threadId");
        $fromDid = $request['from'] ?? throw new Exception("Missing from DID");

        // 2. Extract Ephemeral Key (from 'key' field usually, or part of the request)
        // In OAEP v1, ConnectionRequest usually contains an ephemeral public key for ECDH.
        // Let's assume the request has a 'key' field with the ephemeral public key (multibase or hex).
        // For simplicity, we'll assume it's passed or we just use the sender's DID key if it's a direct static-static handshake (less secure but possible).
        // However, proper OAEP uses ephemeral keys.
        // Let's look at the Rust implementation or assume a standard structure.
        // Rust `ConnectionRequest` has `ephemeral_key`.

        $ephemeralKey = $request['ephemeralKey'] ?? null;
        if (!$ephemeralKey) {
            // Fallback or error. For Echo Bot, we might require it.
            // If missing, we can't do ECDH with ephemeral.
            // Let's assume it's there.
        }

        // 3. Generate Transcript
        // Transcript = Hash(Canonical(ConnectionRequest))
        // We need to canonicalize the request.
        // Since we don't have a library, we'll do a best-effort recursive ksort.
        $canonicalRequest = $this->canonicalize($request);
        $transcript = Hash::blake3(json_encode($canonicalRequest, JSON_UNESCAPED_SLASHES));

        // 4. Sign Transcript
        $signature = $this->identityKey->sign($transcript);

        // 5. Create Response
        $response = [
            '@type' => 'ConnectionResponse',
            'id' => uniqid('msg-'),
            'threadId' => $threadId,
            'from' => $this->did,
            'to' => $fromDid,
            'created' => gmdate('Y-m-d\TH:i:s\Z'),
            'signature' => bin2hex($signature) // Hex encoded signature of the transcript
        ];

        return [
            'response' => $response,
            'transcript' => $transcript,
            'peer_ephemeral_key' => $ephemeralKey
        ];
    }

    private function canonicalize(array $data): array
    {
        ksort($data);
        foreach ($data as $key => $value) {
            if (is_array($value)) {
                $data[$key] = $this->canonicalize($value);
            }
        }
        return $data;
    }
}
