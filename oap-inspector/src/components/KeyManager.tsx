import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export const KeyManager: React.FC = () => {
    const [keyInput, setKeyInput] = useState('');
    const [keys, setKeys] = useState<string[]>([]);

    const handleInject = async () => {
        if (!keyInput) return;
        try {
            await invoke('add_ephemeral_secret', { key: keyInput });
            setKeys(prev => [...prev, keyInput]); // Ideally fetch list from backend
            setKeyInput('');
            alert('Key injected successfully!');
        } catch (e) {
            alert('Failed to inject key: ' + e);
        }
    };

    const handleRemove = async (k: string) => {
        try {
            await invoke('remove_ephemeral_secret', { key: k });
            setKeys(prev => prev.filter(x => x !== k));
        } catch (e) {
            console.error(e);
        }
    };

    return (
        <div className="p-4 border rounded bg-slate-800 text-white mt-4">
            <h3 className="font-bold mb-2">Key Injection (X-Ray)</h3>
            <div className="flex gap-2 mb-2">
                <input
                    type="text"
                    value={keyInput}
                    onChange={e => setKeyInput(e.target.value)}
                    placeholder="Paste Ephemeral Private Key (Multibase)"
                    className="flex-1 p-1 bg-slate-700 rounded border border-slate-600"
                />
                <button onClick={handleInject} className="px-3 py-1 bg-blue-600 rounded hover:bg-blue-500">
                    Inject
                </button>
            </div>
            <div className="text-sm">
                <h4 className="font-semibold text-slate-400">Active Keys:</h4>
                <ul className="list-disc pl-4">
                    {keys.map((k, i) => (
                        <li key={i} className="flex justify-between items-center">
                            <span className="truncate max-w-[200px]">{k.substring(0, 16)}...</span>
                            <button onClick={() => handleRemove(k)} className="text-red-400 hover:text-red-300 ml-2">
                                Remove
                            </button>
                        </li>
                    ))}
                    {keys.length === 0 && <li className="text-slate-500 italic">No keys injected</li>}
                </ul>
            </div>
        </div>
    );
};
