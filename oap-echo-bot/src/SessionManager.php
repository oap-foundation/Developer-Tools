<?php

namespace OAP\EchoBot;

class SessionManager
{
    private array $sessions = [];

    public function createSession(string $threadId, string $peerDid): void
    {
        $this->sessions[$threadId] = [
            'peer_did' => $peerDid,
            'state' => 'AWAIT_RESPONSE',
            'created_at' => time()
        ];
    }

    public function getSession(string $threadId): ?array
    {
        return $this->sessions[$threadId] ?? null;
    }

    public function updateSession(string $threadId, array $data): void
    {
        if (isset($this->sessions[$threadId])) {
            $this->sessions[$threadId] = array_merge($this->sessions[$threadId], $data);
        }
    }

    public function setSessionActive(string $threadId, string $rxKey, string $txKey): void
    {
        $this->updateSession($threadId, [
            'state' => 'ACTIVE',
            'rx_key' => $rxKey,
            'tx_key' => $txKey
        ]);
    }
}
