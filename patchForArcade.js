const fs = require("fs");
const path = require("path");

function replaceCheck(original, search, replace) {
  const result = original.replace(search, replace);
  if (result === original) {
    throw new Error(`Patch ${search} -> ${replace} failed.`);
  }
  return result;
}

function insertAfter(original, search, toInsert) {
  const index = original.indexOf(search);

  if (index === -1) {
    throw new Error(`Patch failed for: ${toInsert}`);
  }

  const pos = index + search.length;
  return original.substr(0, pos) + toInsert + original.substr(pos);
}

function main() {
  const releaseHtmlPath = path.resolve(
    __dirname,
    "release/find-the-dogchicken-web.html"
  );

  let content = fs.readFileSync(releaseHtmlPath, { encoding: "utf-8" });

  // X
  content = replaceCheck(
    content,
    "(e[0].pressed||e[3].pressed||e[5].pressed||e[7].pressed)",
    "(e[2].pressed||e[4].pressed)"
  );

  // Z
  content = replaceCheck(
    content,
    "(e[1].pressed||e[2].pressed||e[4].pressed||e[6].pressed)",
    "(e[1].pressed||e[3].pressed)"
  );

  // exit
  content = replaceCheck(
    content,
    "e[9].pressed&&(this.showMenu=!0)",
    "(e[5].pressed||e[10].pressed)&&(window.customExit())"
  );

  // exit script
  content = insertAfter(
    content,
    "<body>",
    "\n  <script>window.customExit = () => { window.__TAURI_INVOKE__({ cmd: 'tauri', __tauriModule: 'Process', message: { cmd: 'exit', exitCode: 0 }}) }</script>\n"
  );

  fs.writeFileSync(releaseHtmlPath, content, { encoding: "utf-8" });

  console.log("patchForArcade.js successful!");
}

main();
