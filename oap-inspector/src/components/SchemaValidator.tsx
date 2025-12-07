import React, { useState, useEffect } from 'react';
import Ajv from "ajv";
import { getSchemaForType } from "../schemas/oacp";

const ajv = new Ajv({ allErrors: true });

interface SchemaValidatorProps {
    bodyStr?: string; // Decrypted JSON string
}

export const SchemaValidator: React.FC<SchemaValidatorProps> = ({ bodyStr }) => {
    const [validationResult, setValidationResult] = useState<{ valid: boolean, errors?: any[], schemaName?: string } | null>(null);

    useEffect(() => {
        if (!bodyStr) {
            setValidationResult(null);
            return;
        }

        try {
            const data = JSON.parse(bodyStr);
            const type = data.type;

            if (!type) {
                setValidationResult({ valid: false, errors: [{ message: "Property 'type' missing in payload" }] });
                return;
            }

            const schema = getSchemaForType(type);
            if (!schema) {
                setValidationResult({ valid: true, schemaName: "Unknown (No Schema Found)", errors: [] }); // Or warning
                return;
            }

            // AJV Validation
            const validate = ajv.compile(schema);
            const valid = validate(data);

            setValidationResult({
                valid,
                errors: validate.errors || [],
                schemaName: type
            });

        } catch (e) {
            setValidationResult({ valid: false, errors: [{ message: "Invalid JSON: " + e }] });
        }
    }, [bodyStr]);

    if (!validationResult) return null;

    if (validationResult.schemaName === "Unknown (No Schema Found)") {
        return (
            <div className="border border-yellow-600 bg-yellow-900/20 p-2 text-yellow-500 text-xs mb-2 rounded">
                ⚠ Unknown Message Type. No schema definitions found.
            </div>
        );
    }

    if (validationResult.valid) {
        return (
            <div className="border border-green-600 bg-green-900/20 p-2 text-green-400 text-xs mb-2 rounded flex items-center gap-2">
                <span>✔ Valid OACP Message</span>
                <span className="opacity-75 text-[10px]">({validationResult.schemaName})</span>
            </div>
        );
    }

    return (
        <div className="border border-red-600 bg-red-900/20 p-2 text-red-300 text-xs mb-2 rounded">
            <div className="font-bold flex items-center gap-2 mb-1">
                <span>❌ Schema Validation Failed</span>
                <span className="opacity-75 text-[10px]">({validationResult.schemaName})</span>
            </div>
            <ul className="list-disc pl-4 space-y-0.5">
                {validationResult.errors?.map((err, i) => (
                    <li key={i}>{err.instancePath || "Root"} {err.message}</li>
                ))}
            </ul>
        </div>
    );
};
