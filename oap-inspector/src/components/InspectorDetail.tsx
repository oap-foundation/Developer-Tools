import React, { useState, useEffect } from "react";
import { TrafficLog } from "../types";
import Editor from "@monaco-editor/react";
import { HandshakeVisualizer } from "./HandshakeVisualizer";
import { FlowVisualizer } from "./FlowVisualizer";
import { SchemaValidator } from "./SchemaValidator";
import { CredentialViewer } from "./CredentialViewer";

interface InspectorDetailProps {
    log: TrafficLog;
    logs: TrafficLog[];
    onReplay?: (id: number, newBody: string) => void;
}

const InspectorDetail: React.FC<InspectorDetailProps> = ({ log, logs }) => {
    const [activeTab, setActiveTab] = useState<"req" | "res" | "dec_req" | "dec_res" | "viz" | "flow">("req");

    return (
        <div className="flex flex-col h-full bg-[#1e1e1e]">
            {/* Header Info */}
            <div className="p-4 border-b border-[#333] bg-[#252526]">
                <div className="flex items-center space-x-2 mb-2">
                    <span className="font-bold text-lg">{log.method}</span>
                    <span className={`px-2 py-0.5 rounded text-sm ${log.status && log.status >= 400 ? 'bg-red-900 text-red-200' : 'bg-green-900 text-green-200'}`}>
                        {log.status || "PENDING"}
                    </span>
                    <span className="text-gray-500 text-sm ml-auto">{new Date(log.timestamp).toLocaleString()}</span>
                </div>
                <div className="text-sm font-mono text-gray-400 break-all select-all">
                    {log.url}
                </div>
            </div>

            {/* Tabs */}
            <div className="flex bg-[#252526] border-b border-[#333] overflow-x-auto">
                <TabButton active={activeTab === "req"} onClick={() => setActiveTab("req")}>Request</TabButton>
                <TabButton active={activeTab === "res"} onClick={() => setActiveTab("res")}>Response</TabButton>

                {log.decrypted_request_body && (
                    <TabButton active={activeTab === "dec_req"} onClick={() => setActiveTab("dec_req")}>
                        Req (Decrypted)
                    </TabButton>
                )}

                {log.decrypted_response_body && (
                    <TabButton active={activeTab === "dec_res"} onClick={() => setActiveTab("dec_res")}>
                        Res (Decrypted)
                    </TabButton>
                )}

                <TabButton active={activeTab === "viz"} onClick={() => setActiveTab("viz")}>Handshake</TabButton>
                <TabButton active={activeTab === "flow"} onClick={() => setActiveTab("flow")}>Business Flow</TabButton>
            </div>

            {/* Content */}
            <div className="flex-1 overflow-hidden relative flex flex-col">
                {activeTab === "req" && (
                    <MessageViewer headers={log.request_headers} body={log.request_body} />
                )}
                {activeTab === "res" && (
                    <MessageViewer headers={log.response_headers || ""} body={log.response_body} />
                )}

                {/* Enhanced Views for Decrypted Content */}
                {activeTab === "dec_req" && log.decrypted_request_body && (
                    <DecryptedMessageViewer body={log.decrypted_request_body} headers="// Decrypted content" />
                )}
                {activeTab === "dec_res" && log.decrypted_response_body && (
                    <DecryptedMessageViewer body={log.decrypted_response_body} headers="// Decrypted content" />
                )}

                {activeTab === "viz" && (
                    <div className="h-full overflow-auto bg-white">
                        <HandshakeVisualizer logs={logs} selectedLog={log} />
                    </div>
                )}
                {activeTab === "flow" && (
                    <div className="h-full overflow-hidden bg-slate-50 relative">
                        <div className="absolute top-2 left-2 z-10 bg-white/80 p-2 rounded shadow text-black text-xs">
                            <p className="font-bold">OACP Process View</p>
                            <p>Visualizing flow based on `threadId`</p>
                        </div>
                        <FlowVisualizer logs={logs} selectedLog={log} />
                    </div>
                )}
            </div>
        </div>
    );
};

const TabButton: React.FC<{ active: boolean; onClick: () => void; children: React.ReactNode }> = ({ active, onClick, children }) => (
    <button
        onClick={onClick}
        className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap
            ${active ? "border-blue-500 text-white bg-[#1e1e1e]" : "border-transparent text-gray-500 hover:text-gray-300 hover:bg-[#2a2d2e]"}
        `}
    >
        {children}
    </button>
);

const DecryptedMessageViewer: React.FC<{ body: string, headers: string, logId?: number, onReplay?: (id: number, newBody: string) => void }> = ({ body, headers, logId, onReplay }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedBody, setEditedBody] = useState(body);

    useEffect(() => {
        setEditedBody(body);
    }, [body]);

    const handleReplay = () => {
        if (logId && onReplay) {
            onReplay(logId, editedBody);
            setIsEditing(false);
        }
    };

    return (
        <div className="flex flex-col h-full bg-[#1e1e1e]">
            {/* Validators at Top */}
            <div className="p-2 bg-[#252526] flex items-center justify-between">
                <div className="flex-1">
                    <SchemaValidator bodyStr={body} />
                </div>
                {/* Replay Controls (Only for Request) */}
                {logId && onReplay && (
                    <div className="ml-4 flex gap-2">
                        {!isEditing ? (
                            <button
                                onClick={() => setIsEditing(true)}
                                className="px-3 py-1 bg-blue-600 hover:bg-blue-500 text-white text-xs rounded uppercase font-bold"
                            >
                                Edit & Resend
                            </button>
                        ) : (
                            <>
                                <button
                                    onClick={() => { setIsEditing(false); setEditedBody(body); }}
                                    className="px-3 py-1 bg-gray-600 hover:bg-gray-500 text-white text-xs rounded uppercase font-bold"
                                >
                                    Cancel
                                </button>
                                <button
                                    onClick={handleReplay}
                                    className="px-3 py-1 bg-red-600 hover:bg-red-500 text-white text-xs rounded uppercase font-bold animate-pulse"
                                >
                                    Resend (Replay)
                                </button>
                            </>
                        )}
                    </div>
                )}
            </div>

            {/* Main Editor */}
            <div className="flex-1 min-h-0">
                <MessageViewer
                    headers={headers}
                    body={isEditing ? editedBody : body}
                    isDecrypted={true}
                    readOnly={!isEditing}
                    onChange={setEditedBody}
                />
            </div>

            {/* Credential Viewer at Bottom (if VCs exist) */}
            {!isEditing && <CredentialViewer bodyStr={body} />}
        </div>
    );
}

const MessageViewer: React.FC<{ headers: string; body?: string, isDecrypted?: boolean, readOnly?: boolean, onChange?: (val: string) => void }> = ({ headers, body, isDecrypted, readOnly = true, onChange }) => {
    const [viewMode, setViewMode] = useState<"body" | "headers">("body");
    const [decodedBody, setDecodedBody] = useState<string>("");
    const [language, setLanguage] = useState("json");

    useEffect(() => {
        // If editing, body is already raw string from props
        if (!body) {
            setDecodedBody("");
            return;
        }

        try {
            // Only format if valid JSON
            const json = JSON.parse(body);
            setDecodedBody(JSON.stringify(json, null, 2));
            setLanguage("json");
        } catch {
            setDecodedBody(body);
            setLanguage("text");
        }
    }, [body]);

    const handleEditorChange = (value: string | undefined) => {
        if (onChange && value) {
            onChange(value);
        }
    };

    return (
        <div className="flex flex-col h-full">
            <div className="flex border-b border-[#333] bg-[#1e1e1e]">
                <button
                    onClick={() => setViewMode("body")}
                    className={`px-4 py-1 text-xs uppercase tracking-wider ${viewMode === "body" ? "text-blue-400" : "text-gray-500"}`}
                >
                    Body {readOnly ? "" : "(Editing)"}
                </button>
                {!isDecrypted && (
                    <button
                        onClick={() => setViewMode("headers")}
                        className={`px-4 py-1 text-xs uppercase tracking-wider ${viewMode === "headers" ? "text-blue-400" : "text-gray-500"}`}
                    >
                        Headers
                    </button>
                )}
            </div>

            <div className="flex-1 min-h-0 bg-[#1e1e1e]">
                {viewMode === "headers" && !isDecrypted ? (
                    <div className="p-4 font-mono text-xs text-gray-300 whitespace-pre-wrap overflow-auto h-full">
                        {headers}
                    </div>
                ) : (
                    <Editor
                        height="100%"
                        defaultLanguage={language}
                        language={language}
                        value={decodedBody} // This formats it. But editing formatted JSON is annoying if we save formatted. We want to save condensed? Or normalized.
                        // For this demo, let's assume editing the formatted version is fine, but we should parse/minify before sending?
                        // `replay_request` re-encrypts the string. 
                        // If we send pretty-printed JSON, it's fine.
                        onChange={handleEditorChange}
                        theme="vs-dark"
                        options={{
                            minimap: { enabled: false },
                            readOnly: readOnly,
                            fontSize: 12,
                            scrollBeyondLastLine: false,
                        }}
                    />
                )}
            </div>
        </div>
    )
}

export default InspectorDetail;
