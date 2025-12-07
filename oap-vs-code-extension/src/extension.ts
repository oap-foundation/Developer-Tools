import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

// WASM interface (matching Rust export)
interface WasmModule {
    resolve_did(did: string): string;
    generate_identity(): string;
}

let wasm: WasmModule | undefined;
let localNetStatusBar: vscode.StatusBarItem;
let outputChannel: vscode.OutputChannel;

export async function activate(context: vscode.ExtensionContext) {
    console.log('OAP VS Code Extension is now active!');

    outputChannel = vscode.window.createOutputChannel('OAP LocalNet');
    context.subscriptions.push(outputChannel);

    // Initial Status Check
    updateLocalNetStatus(false);

    try {
        // Load WASM module (built to ./out/wasm by docker/script)
        // In local dev without full build, this might fail unless mocked or built manually
        // For nodejs target, standard require works
        wasm = require('../out/wasm/oap_vscode_wasm.js');
        console.log('OAP WASM loaded successfully.');
    } catch (e) {
        console.error('Failed to load OAP WASM module:', e);
        vscode.window.showWarningMessage('OAP WASM module not loaded. Advanced features disabled.');
    }

    // MS 2.2: DID Hover Provider
    const hoverProvider = vscode.languages.registerHoverProvider({ scheme: 'file', language: 'json' }, {
        provideHover(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken) {
            const range = document.getWordRangeAtPosition(position, /did:[a-zA-Z0-9.\-_:]+/);
            if (!range) { return; }

            const word = document.getText(range);
            if (word.startsWith('did:')) {
                if (wasm) {
                    try {
                        const jsonStr = wasm.resolve_did(word);
                        const details = JSON.parse(jsonStr);

                        const md = new vscode.MarkdownString();
                        md.appendMarkdown(`**OAP DID Resolution**\n\n`);
                        md.appendMarkdown(`- **DID**: \`${details.did}\`\n`);
                        md.appendMarkdown(`- **Method**: ${details.method}\n`);
                        md.appendMarkdown(`- **Status**: ${details.status} ✅\n`);
                        md.appendMarkdown(`- **Created**: ${details.created}\n`);

                        return new vscode.Hover(md);
                    } catch (e) {
                        return new vscode.Hover(`Failed to resolve DID with WASM: ${e}`);
                    }
                } else {
                    return new vscode.Hover(`OAP DID: ${word} (WASM not loaded)`);
                }
            }
        }
    });
    context.subscriptions.push(hoverProvider);

    // MS 2.3: Key Generator Command
    const disposable = vscode.commands.registerCommand('oap.generateIdentity', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('Open a file to insert identity.');
            return;
        }

        if (!wasm) {
            vscode.window.showErrorMessage('WASM module not loaded. Cannot generate identity.');
            return;
        }

        try {
            const jsonStr = wasm.generate_identity();
            const identity = JSON.parse(jsonStr);

            // Insert just the DID, or maybe a full block? Prompt implied "embed DID at cursor".
            // Let's insert the DID string for now, or maybe a snippet.
            // "Erzeugt ein Schlüsselpaar und fügt die DID direkt an der Cursor-Position im Code ein."
            // -> Inserting DID.

            editor.edit((editBuilder: vscode.TextEditorEdit) => {
                editBuilder.insert(editor.selection.active, identity.did);
            });

            vscode.window.showInformationMessage(`Generated Identity: ${identity.did} (Private Key hidden)`);
        } catch (e) {
            vscode.window.showErrorMessage(`Error generating identity: ${e}`);
        }
    });

    context.subscriptions.push(disposable);

    // MS 3.1: LocalNet Controller
    localNetStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    localNetStatusBar.command = 'oap.toggleLocalNet';
    context.subscriptions.push(localNetStatusBar);
    updateLocalNetStatus(false);
    localNetStatusBar.show();

    context.subscriptions.push(vscode.commands.registerCommand('oap.toggleLocalNet', () => {
        toggleLocalNet();
    }));

    // MS 3.2: Interaction Simulation (Send to Relay)
    context.subscriptions.push(vscode.commands.registerCommand('oap.sendToRelay', (uri: vscode.Uri) => {
        sendToRelay(uri);
    }));

    // MS 3.3: Project Scaffolding
    context.subscriptions.push(vscode.commands.registerCommand('oap.createAgentProject', () => {
        createAgentProject();
    }));

    // Phase 4: Welcome Page
    context.subscriptions.push(vscode.commands.registerCommand('oap.showWelcome', () => {
        showWelcomePage(context);
    }));

    // First Run Detection
    const hasshownWelcome = context.globalState.get('oap.hasShownWelcome', false);
    if (!hasshownWelcome) {
        showWelcomePage(context);
        context.globalState.update('oap.hasShownWelcome', true);
    }
}

function showWelcomePage(context: vscode.ExtensionContext) {
    const panel = vscode.window.createWebviewPanel(
        'oapWelcome',
        'Welcome to OAP',
        vscode.ViewColumn.One,
        { enableScripts: true }
    );

    // Simple HTML content with embedded OAP branding/info
    panel.webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to OAP</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; padding: 20px; line-height: 1.6; }
        h1 { color: #007acc; }
        .feature { margin-bottom: 15px; padding: 10px; border: 1px solid #ccc; border-radius: 5px; }
        .feature h3 { margin-top: 0; }
        a { color: #007acc; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>Welcome to the Open Agent Protocol (OAP) Extension!</h1>
    <p>We're excited to help you build the next generation of autonomous agents.</p>
    
    <h2>⚡ Quick Start</h2>
    <div class="feature">
        <h3>1. Create a Project</h3>
        <p>Run <code>OAP: Create New Agent Project</code> in the Command Palette.</p>
    </div>
    <div class="feature">
        <h3>2. Edit Intelligence</h3>
        <p>Use snippets like <code>oacp-offer</code> and hover over DIDs to see details.</p>
    </div>
    <div class="feature">
        <h3>3. Simulate & Test</h3>
        <p>Use the Status Bar to start the <strong>LocalNet</strong> and right-click files to <strong>Send to Relay</strong>.</p>
    </div>

    <h2>📚 Resources</h2>
    <ul>
        <li><a href="https://github.com/oap-foundation">OAP Foundation GitHub</a></li>
        <li><a href="https://github.com/oap-foundation/oap-tools">Developer Tools Repo</a></li>
    </ul>
</body>
</html>`;
}

// Helper to execute commands safely
function safeSpawn(command: string, args: string[], cwd?: string): Promise<string> {
    return new Promise((resolve, reject) => {
        outputChannel.appendLine(`Spawning: ${command} ${args.join(' ')} in ${cwd || 'default cwd'}`);

        const process = cp.spawn(command, args, { cwd, shell: true });

        let stdout = '';
        let stderr = '';

        process.stdout.on('data', (data: any) => {
            const str = data.toString();
            outputChannel.append(str);
            stdout += str;
        });

        process.stderr.on('data', (data: any) => {
            const str = data.toString();
            outputChannel.append(str); // Log stderr too
            stderr += str;
        });

        process.on('close', (code: number) => {
            if (code === 0) {
                resolve(stdout.trim());
            } else {
                reject(new Error(`Process exited with code ${code}: ${stderr}`));
            }
        });

        process.on('error', (err: any) => {
            reject(err);
        });
    });
}

function updateLocalNetStatus(isOnline: boolean) {
    if (isOnline) {
        localNetStatusBar.text = '$(circle-filled) OAP LocalNet: Online';
        localNetStatusBar.color = '#00FF00'; // Greenish
    } else {
        localNetStatusBar.text = '$(circle-outline) OAP LocalNet: Offline';
        localNetStatusBar.color = undefined;
    }
}

// Helper to get configured LocalNet path
function getLocalNetPath(): string {
    const config = vscode.workspace.getConfiguration('oap');
    const configPath = config.get('localNetPath') || '../oap-localnet';
    if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
        return path.resolve(vscode.workspace.workspaceFolders[0].uri.fsPath, configPath as string);
    }
    return configPath as string;
}

async function toggleLocalNet() {
    const isOnline = localNetStatusBar.text.includes('Online');
    const cwd = getLocalNetPath();
    const args = isOnline ? ['compose', 'down'] : ['compose', 'up', '-d'];

    try {
        await safeSpawn('docker', args, cwd);
        updateLocalNetStatus(!isOnline);
        vscode.window.showInformationMessage(`LocalNet is now ${!isOnline ? 'Online' : 'Offline'}`);
    } catch (e: any) {
        vscode.window.showErrorMessage(`LocalNet Error: ${e.message}`);
    }
}

async function sendToRelay(uri?: vscode.Uri) {
    if (!uri && vscode.window.activeTextEditor) {
        uri = vscode.window.activeTextEditor.document.uri;
    }

    if (!uri) {
        vscode.window.showErrorMessage('No file selected to send.');
        return;
    }

    try {
        const content = fs.readFileSync(uri.fsPath, 'utf-8');
        const json = JSON.parse(content);
        if (!json['@context']) {
            vscode.window.showWarningMessage('File does not look like an OAP message (missing @context).');
        }

        const config = vscode.workspace.getConfiguration('oap');
        const relayUrl = config.get('relayUrl') || 'http://localhost:8000';

        outputChannel.appendLine(`Sending ${uri.fsPath} to ${relayUrl}...`);

        // Use curl via spawn for better argument handling
        // Note: 'data-binary @file' is specific to curl, here we pipe content or use file arg
        // safeSpawn sanitizes args array, but curl expects file path with @

        await safeSpawn('curl', ['-X', 'POST', '-H', 'Content-Type: application/json', '-d', `@${uri.fsPath}`, relayUrl]);

        vscode.window.showInformationMessage('Message sent to Relay successfully!');
    } catch (e: any) {
        vscode.window.showErrorMessage(`Error: ${e.message}`);
    }
}

async function createAgentProject() {
    const projectName = await vscode.window.showInputBox({
        prompt: 'Project Name',
        placeHolder: 'my-oap-agent'
    });
    if (!projectName) { return; }

    const template = await vscode.window.showQuickPick(['Node.js Starter', 'Python Flask Agent'], {
        placeHolder: 'Select a template'
    });
    if (!template) { return; }

    const folderUris = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: 'Select Parent Folder'
    });

    if (!folderUris || folderUris.length === 0) { return; }

    const targetDir = path.join(folderUris[0].fsPath, projectName);
    fs.mkdirSync(targetDir, { recursive: true });

    // Create dummy files based on template
    if (template === 'Node.js Starter') {
        fs.writeFileSync(path.join(targetDir, 'package.json'), `{\n  "name": "${projectName}",\n  "version": "1.0.0",\n  "dependencies": { "oap-sdk": "latest" }\n}`);
        fs.writeFileSync(path.join(targetDir, 'agent.js'), `// OAP Agent\nconsole.log("Agent ${projectName} started");`);
    } else {
        fs.writeFileSync(path.join(targetDir, 'requirements.txt'), `oap-sdk`);
        fs.writeFileSync(path.join(targetDir, 'agent.py'), `# OAP Agent\nprint("Agent ${projectName} started")`);
    }

    vscode.window.showInformationMessage(`Project ${projectName} created!`);

    // Open the new project
    const uri = vscode.Uri.file(targetDir);
    vscode.commands.executeCommand('vscode.openFolder', uri, true);
}

export function deactivate() { }
