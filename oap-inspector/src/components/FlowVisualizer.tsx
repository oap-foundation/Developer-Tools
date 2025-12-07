import React, { useMemo } from 'react';
import ReactFlow, {
    Node,
    Edge,
    Controls,
    Background,
    MarkerType
} from 'reactflow';
import 'reactflow/dist/style.css';
import { TrafficLog } from "../types";

interface FlowVisualizerProps {
    logs: TrafficLog[];
    selectedLog: TrafficLog;
}

export const FlowVisualizer: React.FC<FlowVisualizerProps> = ({ logs, selectedLog }) => {
    // 1. Extract Thread ID from selected Log
    const threadId = useMemo(() => {
        if (!selectedLog || !selectedLog.decrypted_request_body) return null;
        try {
            const body = JSON.parse(selectedLog.decrypted_request_body);
            return body.threadId;
        } catch { return null; }
    }, [selectedLog]);

    // 2. Build Graph Data
    const { nodes, edges } = useMemo(() => {
        if (!threadId) return { nodes: [], edges: [] };

        const relatedLogs = logs.filter(l => {
            // Check decrypted bodies for matching threadId
            const check = (bodyStr?: string) => {
                if (!bodyStr) return false;
                try {
                    const b = JSON.parse(bodyStr);
                    return b.threadId === threadId;
                } catch { return false; }
            };
            return check(l.decrypted_request_body) || check(l.decrypted_response_body);
        }).sort((a, b) => a.id - b.id);

        const flowNodes: Node[] = [];
        const flowEdges: Edge[] = [];

        relatedLogs.forEach((log, index) => {
            // Simplification: Each Log Entry is a Step.
            // Ideally we parse Request AND Response as separate steps if they both have threadId.
            // Let's inspect Request Type.
            let type = "Unknown";
            let label = `Msg #${log.id}`;

            try {
                if (log.decrypted_request_body) {
                    const b = JSON.parse(log.decrypted_request_body);
                    if (b.type) {
                        // Extract last part of type URL
                        type = b.type.split('/').pop() || b.type;
                        label = type;
                    }
                }
            } catch { }

            // Color coding based on type
            let bg = "#fff";
            if (label.includes("offer")) bg = "#eff6ff"; // blue-50
            if (label.includes("order")) bg = "#f0fdf4"; // green-50
            if (label.includes("invoice")) bg = "#fefce8"; // yellow-50
            if (label.includes("payment")) bg = "#ecfccb"; // lime-100

            // Highlight selected
            const isSelected = log.id === selectedLog.id;

            flowNodes.push({
                id: log.id.toString(),
                position: { x: 250, y: index * 100 + 50 },
                data: { label: label },
                style: {
                    background: bg,
                    border: isSelected ? '2px solid #3b82f6' : '1px solid #777',
                    padding: 10,
                    borderRadius: 5,
                    width: 150,
                    textAlign: 'center'
                },
                type: 'default' // or custom
            });

            if (index > 0) {
                flowEdges.push({
                    id: `e${index - 1}-${index}`,
                    source: relatedLogs[index - 1].id.toString(),
                    target: log.id.toString(),
                    markerEnd: { type: MarkerType.ArrowClosed },
                    animated: true,
                });
            }
        });

        return { nodes: flowNodes, edges: flowEdges };
    }, [logs, threadId, selectedLog]);

    if (!threadId) {
        return (
            <div className="p-8 text-center text-slate-500">
                <p>No high-level flow detected (`threadId` missing).</p>
                <p className="text-sm mt-2">Select a decrypted OACP message to visualize.</p>
            </div>
        );
    }

    return (
        <div className="h-full w-full bg-slate-50">
            <ReactFlow nodes={nodes} edges={edges} fitView>
                <Background />
                <Controls />
            </ReactFlow>
        </div>
    );
};
