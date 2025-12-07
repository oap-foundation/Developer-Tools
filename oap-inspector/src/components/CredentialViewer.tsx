import React, { useMemo } from 'react';

interface CredentialViewerProps {
    bodyStr?: string;
}

export const CredentialViewer: React.FC<CredentialViewerProps> = ({ bodyStr }) => {

    // Extract credentials from payload
    const credentials = useMemo(() => {
        if (!bodyStr) return [];
        try {
            const data = JSON.parse(bodyStr);
            // Look for `credential`, `verifiableCredential`, or inside `presentation`
            // Simplification: Check common paths
            const list = [];
            if (data.verifiableCredential) list.push(...(Array.isArray(data.verifiableCredential) ? data.verifiableCredential : [data.verifiableCredential]));
            if (data.credential) list.push(data.credential);
            return list;
        } catch { return []; }
    }, [bodyStr]);

    if (credentials.length === 0) return null;

    return (
        <div className="bg-slate-800 p-4 border-t border-slate-700">
            <h4 className="text-sm font-bold text-slate-300 mb-2">Verifiable Credentials Found ({credentials.length})</h4>
            <div className="flex flex-wrap gap-4">
                {credentials.map((vc, i) => (
                    <CredentialCard key={i} vc={vc} />
                ))}
            </div>
        </div>
    );
};

const CredentialCard: React.FC<{ vc: any }> = ({ vc }) => {
    // Basic W3C VC parsing
    const issuer = typeof vc.issuer === 'string' ? vc.issuer : vc.issuer?.id || "Unknown";
    const types = Array.isArray(vc.type) ? vc.type.join(", ") : vc.type;
    const subject = vc.credentialSubject?.id || "Unknown Subject";
    // Photo?
    const photo = vc.credentialSubject?.image || vc.credentialSubject?.photo;

    // Simplified Trust Logic (Mock)
    const isTrusted = issuer.includes("oap.foundation") || issuer.includes("trusted");

    return (
        <div className="w-[320px] bg-slate-100 text-slate-900 rounded-xl overflow-hidden shadow-lg border-2 border-slate-300 relative">
            {/* Header / Banner */}
            <div className={`h-16 flex items-center px-4 ${isTrusted ? 'bg-blue-600' : 'bg-slate-600'}`}>
                <span className="text-white font-bold tracking-widest uppercase text-xs">
                    {isTrusted ? "Verified Issuer" : "Unknown Issuer"}
                </span>
            </div>

            <div className="p-4 relative">
                {/* Photo ID style */}
                <div className="flex gap-4">
                    <div className="w-16 h-16 bg-slate-300 rounded-md flex-shrink-0 overflow-hidden">
                        {photo ? <img src={photo} alt="ID" className="w-full h-full object-cover" /> : <div className="flex items-center justify-center h-full text-xs text-slate-500">No Photo</div>}
                    </div>
                    <div className="overflow-hidden">
                        <div className="text-xs text-slate-500 uppercase font-bold">Type</div>
                        <div className="text-sm font-bold truncate mb-2" title={types}>{types}</div>

                        <div className="text-xs text-slate-500 uppercase font-bold">Subject</div>
                        <div className="text-sm truncate font-mono" title={subject}>{subject.substring(0, 16)}...</div>
                    </div>
                </div>

                <div className="mt-4 border-t border-slate-200 pt-2">
                    <div className="text-xs text-slate-500 uppercase font-bold">Issuer</div>
                    <div className="text-xs truncate text-blue-800" title={issuer}>{issuer}</div>
                </div>
            </div>

            {/* Watermark / Seal */}
            {isTrusted && (
                <div className="absolute top-12 right-4 w-12 h-12 bg-yellow-400 rounded-full flex items-center justify-center shadow-md border-2 border-white">
                    <span className="text-xl">✓</span>
                </div>
            )}
        </div>
    );
};
