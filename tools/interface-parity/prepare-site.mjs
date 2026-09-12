#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const artifacts = path.join(root, ".artifacts/interface-parity/site");
const web = path.join(root, "web");
const marker = "    <script>load(\"open-rebellion.wasm\");</script>";
const fixturePlugin = `    <script>
        (function registerInterfaceFixtureBridge() {
            "use strict";
            const params = new URLSearchParams(window.location.search);
            const values = params.getAll("fixture-code");
            const fixtureCode = values.length === 1 && /^\\d+$/.test(values[0])
                ? Number(values[0])
                : 0;
            function register(imports) {
                imports.env.open_rebellion_interface_fixture_code = function () {
                    return fixtureCode;
                };
                imports.env.open_rebellion_interface_fixture_emit = function (ptr, len) {
                    try {
                        const bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
                        const message = JSON.parse(
                            new TextDecoder("utf-8", { fatal: true }).decode(bytes),
                        );
                        if (message.status === "selected") {
                            window.__openRebellionInterfaceSelection = message;
                        } else if (message.status === "hovered") {
                            window.__openRebellionInterfaceHover = message;
                        } else {
                            window.__openRebellionInterfaceReady = message;
                        }
                    } catch (error) {
                        window.__openRebellionInterfaceReady = {
                            schema_version: 1,
                            status: "failed",
                            error: String(error),
                        };
                    }
                    window.dispatchEvent(new CustomEvent(
                        "open-rebellion-interface-ready",
                        { detail: window.__openRebellionInterfaceReady },
                    ));
                };
            }
            miniquad_add_plugin({
                register_plugin: register,
                version: 1,
                name: "open_rebellion_interface_fixture",
            });
        }());
    </script>
    <script>load("open-rebellion-test.wasm");</script>`;

fs.mkdirSync(path.join(artifacts, "data"), { recursive: true });
for (const [source, destination] of [
  [path.join(web, "gl.js"), path.join(artifacts, "gl.js")],
  [path.join(web, "data/runtime.orpk"), path.join(artifacts, "data/runtime.orpk")],
]) {
  fs.copyFileSync(source, destination);
}

const productionHtml = fs.readFileSync(path.join(web, "index.html"), "utf8");
if (!productionHtml.includes(marker)) {
  throw new Error("production HTML load marker changed; fixture shell was not generated");
}
fs.writeFileSync(
  path.join(artifacts, "index.html"),
  productionHtml.replace(marker, fixturePlugin),
);
