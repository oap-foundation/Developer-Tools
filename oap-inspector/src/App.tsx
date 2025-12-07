import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TrafficLog } from "./types";
import TrafficList from "./components/TrafficList";
import InspectorDetail from "./components/InspectorDetail";
import { KeyManager } from "./components/KeyManager";

function App() {
  const [logs, setLogs] = useState<TrafficLog[]>([]);
  const [selectedLog, setSelectedLog] = useState<TrafficLog | null>(null);

  const fetchLogs = async () => {
    try {
      const result = await invoke<TrafficLog[]>("get_traffic_logs", {});
      setLogs(result);
    } catch (error) {
      console.error("Failed to fetch logs:", error);
    }
  };

  useEffect(() => {
    const interval = setInterval(fetchLogs, 1000); // Poll every second
    fetchLogs(); // Initial fetch
    return () => clearInterval(interval);
  }, []);

  const handleReplay = async (id: number, newBody: string) => {
    try {
      const result = await invoke<string>("replay_request", { id, newDecryptedBody: newBody });
      alert(result); // Simple feedback
      fetchLogs(); // Refresh to see new log
    } catch (e) {
      console.error("Replay failed:", e);
      alert("Replay failed: " + e);
    }
  };

  const handleExport = async () => {
    const path = prompt("Enter file path to export logs (e.g. /tmp/session.json):");
    if (!path) return;
    try {
      const res = await invoke("export_logs", { path });
      alert(res);
    } catch (e) { alert("Export failed: " + e); }
  };

  const handleImport = async () => {
    const path = prompt("Enter file path to import logs from:");
    if (!path) return;
    try {
      const res = await invoke("import_logs", { path });
      alert(res);
      fetchLogs();
    } catch (e) { alert("Import failed: " + e); }
  };

  return (
    <div className="h-screen w-screen bg-[#1e1e1e] text-white flex overflow-hidden">
      {/* Sidebar / List View */}
      <div className="w-1/3 flex flex-col border-r border-[#333]">
        <div className="p-4 border-b border-[#333] bg-[#252526] flex justify-between items-center">
          <h1 className="text-sm font-bold uppercase tracking-wider text-gray-400">Traffic Inspector</h1>
          <div className="flex gap-2">
            <button onClick={handleExport} className="text-xs bg-slate-700 px-2 py-1 rounded hover:bg-slate-600" title="Export Session">📤</button>
            <button onClick={handleImport} className="text-xs bg-slate-700 px-2 py-1 rounded hover:bg-slate-600" title="Import Session">📥</button>
          </div>
        </div>

        {/* Traffic List takes available space */}
        <div className="flex-1 overflow-y-auto">
          <TrafficList
            logs={logs}
            selectedId={selectedLog?.id || null}
            onSelect={setSelectedLog}
          />
        </div>

        {/* Key Manager at bottom */}
        <div className="border-t border-[#333] bg-[#252526] p-2">
          <KeyManager />
        </div>
      </div>

      {/* Main Content / Detail View */}
      <div className="flex-1 flex flex-col h-full">
        {selectedLog ? (
          <InspectorDetail log={selectedLog} logs={logs} onReplay={handleReplay} />
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            Select a request to view details
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
