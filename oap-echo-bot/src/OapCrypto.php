<?php

namespace OAP\EchoBot;

use OAP\OAEP\KeyPair;
use OAP\OAEP\Utils\Hash;
use OAP\OAEP\Utils\Hkdf;
use Exception;

/**
 * Helper class for OAP cryptographic operations.
 * Wraps oaep-php functionality for the Echo Bot.
 */
class OapCrypto
{
    /**
     * Generate a new Ed25519 KeyPair.
     */
    public static function generateKeyPair(): KeyPair
    {
        return KeyPair::generate();
    }

    /**
     * Hash data using BLAKE3.
     */
    public static function hash(string $data): string
    {
        return Hash::blake3($data);
    }

    /**
     * Derive a key using HKDF-SHA256.
     */
    public static function deriveKey(string $ikm, string $salt, string $info, int $length): string
    {
        return Hkdf::derive($ikm, $salt, $info, $length);
    }

    /**
     * Sign a message using Ed25519.
     */
    public static function sign(string $message, KeyPair $keyPair): string
    {
        return $keyPair->sign($message);
    }

    /**
     * Verify an Ed25519 signature.
     */
    public static function verify(string $message, string $signature, string $publicKey): bool
    {
        // oaep-php KeyPair doesn't expose a static verify for raw public key bytes easily
        // without creating a KeyPair object, but we can use sodium directly here for utility.
        return sodium_crypto_sign_verify_detached($signature, $message, $publicKey);
    }
}
