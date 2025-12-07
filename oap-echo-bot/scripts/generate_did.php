<?php

require_once __DIR__ . '/../src/OapCrypto.php';
// Manual autoloader since we can't run composer install yet
require_once __DIR__ . '/../src/OapCrypto.php';
// Manual autoloader since we can't run composer install yet
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/KeyPair.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Did.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/DidKey.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/DidDocument.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/DidWeb.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Utils/Multibase.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Utils/Base58.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Utils/Hash.php';

use OAP\OAEP\DidKey;
use OAP\OAEP\DidDocument;

// 1. Generate KeyPair
echo "Generating new identity...\n";
$didKey = DidKey::generate();
$keyPair = $didKey->getKeyPair();

// 2. Define DID Web
$domain = "echo.oap.foundation";
$didWeb = "did:web:$domain";

echo "DID: $didWeb\n";
echo "Public Key (Ed25519): " . bin2hex($keyPair->publicKey) . "\n";

// 3. Create DID Document
// We reuse DidDocument logic but customize the ID
$doc = DidDocument::forDidKey($didKey);
// Patch the ID to be did:web
$json = json_encode($doc);
$data = json_decode($json, true);

$data['id'] = $didWeb;
// Update verification method ID
$vmId = $data['verificationMethod'][0]['id'];
$newVmId = str_replace($didKey->getDid(), $didWeb, $vmId);
$data['verificationMethod'][0]['id'] = $newVmId;
$data['verificationMethod'][0]['controller'] = $didWeb;

// Update authentication/assertion refs
$data['authentication'] = [$newVmId];
$data['assertionMethod'] = [$newVmId];
$data['keyAgreement'][0] = str_replace($didKey->getDid(), $didWeb, $data['keyAgreement'][0]);
// keyAgreement in forDidKey is just a string ref, not an object with controller.
// If we want to make it an object, we can, but for now let's keep it simple as a ref.

// 4. Save to file
$outputDir = __DIR__ . '/../data';
if (!is_dir($outputDir))
    mkdir($outputDir);

// Save keys (SECRET! In production, use secure storage)
$keys = [
    'did' => $didWeb,
    'secret_key' => bin2hex($keyPair->secretKey),
    'public_key' => bin2hex($keyPair->publicKey)
];
file_put_contents("$outputDir/keys.json", json_encode($keys, JSON_PRETTY_PRINT));
echo "Saved keys to data/keys.json\n";

// Save DID Document
file_put_contents("$outputDir/did.json", json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES));
echo "Saved DID Document to data/did.json\n";
