import * as path from "path";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Executable,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const config = workspace.getConfiguration("alLsp");
  const serverPath = config.get<string>("serverPath") || "al-lsp";

  const serverExecutable: Executable = {
    command: serverPath,
    args: [],
  };

  const serverOptions: ServerOptions = {
    run: serverExecutable,
    debug: serverExecutable,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "al" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.al"),
    },
  };

  client = new LanguageClient(
    "alLsp",
    "AL Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
