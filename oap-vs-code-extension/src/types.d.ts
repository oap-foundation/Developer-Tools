// Helper type definitions to satisfy VS Code when @types/node is missing
declare var require: any;
declare var console: Console;

interface Console {
    log(message?: any, ...optionalParams: any[]): void;
    warn(message?: any, ...optionalParams: any[]): void;
    error(message?: any, ...optionalParams: any[]): void;
}

declare module 'child_process' {
    export function exec(command: string, options: any, callback: (error: Error | null, stdout: string, stderr: string) => void): any;
    export function exec(command: string, callback: (error: Error | null, stdout: string, stderr: string) => void): any;
    export function spawn(command: string, args: string[], options: any): any;
}

declare module 'fs' {
    export function readFileSync(path: string, options: string): string;
    export function writeFileSync(path: string, content: string): void;
    export function mkdirSync(path: string, options?: any): void;
}

declare module 'path' {
    export function join(...paths: string[]): string;
    export function resolve(...paths: string[]): string;
}

// Mock vscode module to silence "Cannot find module 'vscode'" errors in local dev without install
declare module 'vscode' {
    export const window: any;
    export const commands: any;
    export const workspace: any;
    export const languages: any;
    export const StatusBarAlignment: any;
    export const ViewColumn: any;
    export const Uri: any;
    export const MarkdownString: any;
    export const Hover: any;
    export const Position: any;
    export const Range: any;
    export const CancellationToken: any;
    export const TextDocument: any;
    export const TextEditorEdit: any;
    export const ExtensionContext: any;
    export const OutputChannel: any;
    export const StatusBarItem: any;
    export type TextDocument = any;
    export type Position = any;
    export type CancellationToken = any;
    export type TextEditorEdit = any;
    export type ExtensionContext = any;
    export type OutputChannel = any;
    export type StatusBarItem = any;
    export type Uri = any;
}
