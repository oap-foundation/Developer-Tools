import React, { useMemo } from 'react';
import { TrafficLog } from "../types";

interface HandshakeVisualizerProps {
    logs: TrafficLog[];
    selectedLog: TrafficLog;
}

export const HandshakeVisualizer: React.FC<HandshakeVisualizerProps> = ({ logs, selectedLog }) => {
    // 1. Identify Conversation ID
    const conversationId = useMemo(() => {
        if (!selectedLog) return null;
        try {
            const body = JSON.parse(selectedLog.request_body || "{}");
            // If it's a Request, it has 'id'. If Response/Ack, it has 'replyTo' (mapped to camelCase inside JSON?)
            // Rust struct uses 'camelCase' rename.
            return body.id || body.replyTo;
        } catch {
            return null;
        }
    }, [selectedLog]);

    // 2. Filter related logs
    const relatedLogs = useMemo(() => {
        if (!conversationId) return [];
        return logs.filter(l => {
            try {
                // Check request body (proxy stores request body)
                // Wait, TrafficLog has request_body and response_body.
                // The Message is in the Request Body (if POST) or Response Body?
                // ConnectionRequest is POST. Response is HTTP Response Body?
                // Handshake is usually HTTP POST (Request) -> HTTP 200 (Response).
                // So Request Log contains ConnectionRequest. Response Log contains ConnectionResponse.

                // If I am looking at a specific HTTP transaction (Row in TrafficLog), it contains BOTH Request and Response.
                // But Handshake implies MULTIPLE HTTP transactions?
                // 1. Alice POST ConnectionRequest -> Bob returns 200 OK (with ConnectionResponse body? or empty?)
                // OAP over HTTP:
                // Pattern A: Async. Alice POST -> Bob 202. Bob POST Response -> Alice 200.
                // Pattern B: Sync. Alice POST Request -> Bob returns Response in body.

                // "The handshake consists of three messages"
                // If Sync:
                // 1. Alice -> Bob (Request) [HTTP Req]
                //    Bob -> Alice (Response) [HTTP Res]
                // 2. Alice -> Bob (Ack) [HTTP Req]
                //    Bob -> Alice (200 OK) [HTTP Res]

                // So we have 2 TrafficLog entries for a full handshake.
                // Entry 1: Req=ConnReq, Res=ConnRes
                // Entry 2: Req=ConnAck, Res={}

                // Matching logic:
                // Entry 1 Req.id == ConversationID.
                // Entry 2 Req.replyTo == ConversationID.

                // So checking `l.request_body` for `id == cid` OR `replyTo == cid`.

                const reqB = JSON.parse(l.request_body || "{}");
                return reqB.id === conversationId || reqB.replyTo === conversationId;
            } catch { return false; }
        }).sort((a, b) => a.id - b.id);
    }, [logs, conversationId]);

    if (!conversationId || relatedLogs.length === 0) {
        return <div className="p-4 text-slate-400">Select a handshake message to visualize flow.</div>;
    }

    return (
        <div className="p-4 bg-white rounded shadow text-black overflow-auto">
            <h3 className="font-bold mb-4">Handshake Flow (ID: {conversationId.substring(0, 8)}...)</h3>
            <svg width="600" height={relatedLogs.length * 100 + 100} className="mx-auto">
                <defs>
                    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="0" refY="3.5" orient="auto">
                        <polygon points="0 0, 10 3.5, 0 7" fill="#333" />
                    </marker>
                </defs>

                {/* Actors */}
                <text x="50" y="30" fontWeight="bold">Initiator (Alice)</text>
                <line x1="100" y1="40" x2="100" y2="1000" stroke="#ccc" strokeDasharray="4" />

                <text x="450" y="30" fontWeight="bold">Responder (Bob)</text>
                <line x1="500" y1="40" x2="500" y2="1000" stroke="#ccc" strokeDasharray="4" />

                {/* Messages */}
                {relatedLogs.map((log, i) => {
                    const reqBody = JSON.parse(log.request_body || "{}");
                    const resBody = JSON.parse(log.response_body || "{}");

                    const isAck = reqBody.type?.includes("Acknowledge");
                    const y = 80 + i * 100;

                    return (
                        <g key={log.id}>
                            {/* Request Arrow (Alice -> Bob) */}
                            <line x1="100" y1={y} x2="490" y2={y} stroke={isAck ? "#10b981" : "#3b82f6"} strokeWidth="2" markerEnd="url(#arrowhead)" />
                            <text x="300" y={y - 10} textAnchor="middle" fontSize="12" fill="#555">
                                {reqBody.type || "Unknown Request"}
                            </text>

                            {/* Response Arrow (Bob -> Alice) - Only if not empty */}
                            {resBody.type && (
                                <>
                                    <line x1="500" y1={y + 40} x2="110" y2={y + 40} stroke="#f59e0b" strokeWidth="2" markerEnd="url(#arrowhead)" />
                                    <text x="300" y={y + 30} textAnchor="middle" fontSize="12" fill="#555">
                                        {resBody.type || "Response"}
                                    </text>
                                </>
                            )}

                            {/* Status Indicator */}
                            <circle cx="530" cy={y} r="5" fill={log.error ? "red" : "green"} />
                        </g>
                    );
                })}
            </svg>
        </div>
    );
};
