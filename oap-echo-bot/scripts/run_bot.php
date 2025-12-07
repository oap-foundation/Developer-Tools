<?php

require_once __DIR__ . '/../src/OapCrypto.php';
require_once __DIR__ . '/../src/Bot.php';

// Manual autoloader for bindings
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/KeyPair.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Did.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/DidKey.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/SessionKey.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Utils/Hash.php';
require_once __DIR__ . '/../../../Language Bindings/oaep-php/src/Utils/Hkdf.php';

require_once __DIR__ . '/../../../Language Bindings/oatp-php/src/RelayClient.php';
require_once __DIR__ . '/../../../Language Bindings/oatp-php/src/BlindInbox.php';
require_once __DIR__ . '/../../../Language Bindings/oatp-php/src/Relay/RetrieveResponse.php';
require_once __DIR__ . '/../../../Language Bindings/oatp-php/src/Relay/UploadRequest.php';
require_once __DIR__ . '/../../../Language Bindings/oatp-php/src/Sharding/Sharding.php';

use OAP\EchoBot\Bot;

$config = [
    'key_file' => __DIR__ . '/../data/keys.json',
    'relay_url' => 'http://localhost:3000', // Default local relay
    'poll_interval' => 2
];

$bot = new Bot($config);
$bot->start();
