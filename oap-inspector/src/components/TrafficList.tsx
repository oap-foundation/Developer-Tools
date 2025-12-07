import React from "react";
import { TrafficLog } from "../types";
// Utils import removed
// I'll inline 'cn' or equivalent logic here to be self-contained for now, or assume I'll create utils.ts later.
// Let's implement simple clsx/tailwind-merge here or basic template literals.

interface TrafficListProps {
    logs: TrafficLog[];
    selectedId: number | null;
    onSelect: (log: TrafficLog) => void;
}

const TrafficList: React.FC<TrafficListProps> = ({ logs, selectedId, onSelect }) => {
    return (
        <div className="flex-1 overflow-y-auto">
            {logs.map((log) => {
                const isSelected = log.id === selectedId;
                const isError = log.status && log.status >= 400;

                return (
                    <div
                        key={log.id}
                        onClick={() => onSelect(log)}
                        className={`
              flex items-center p-2 text-sm border-b border-[#333] cursor-pointer hover:bg-[#2a2d2e]
              ${isSelected ? "bg-[#37373d] text-white" : "text-gray-300"}
            `}
                    >
                        <div className={`
              w-12 h-5 flex items-center justify-center rounded text-[10px] font-bold mr-2
              ${log.method === "GET" ? "bg-blue-900 text-blue-300" : ""}
              ${log.method === "POST" ? "bg-green-900 text-green-300" : ""}
              ${log.method === "PUT" ? "bg-yellow-900 text-yellow-300" : ""}
              ${log.method === "DELETE" ? "bg-red-900 text-red-300" : ""}
            `}>
                            {log.method}
                        </div>

                        <div className="flex-1 truncate mr-2 font-mono text-xs" title={log.url}>
                            {getMainPath(log.url)}
                        </div>

                        <div className="flex flex-col items-end min-w-[60px]">
                            <span className={`text-xs font-mono mb-1 ${isError ? "text-red-400" : "text-green-400"}`}>
                                {log.status || "..."}
                            </span>
                            <span className="text-[10px] text-gray-500">
                                {log.duration_ms !== undefined ? `${log.duration_ms}ms` : ""}
                            </span>
                        </div>
                    </div>
                );
            })}
        </div>
    );
};

function getMainPath(urlStr: string): string {
    try {
        const u = new URL(urlStr);
        return u.pathname;
    } catch {
        return urlStr;
    }
}

export default TrafficList;
