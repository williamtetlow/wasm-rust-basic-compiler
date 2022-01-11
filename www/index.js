import { parse } from "wasm-basic-compiler";

document.getElementById("code-input").addEventListener("input", (e) => {
  const value = e.currentTarget.value;

  const parseResult = parse(value);

  document.getElementById("code-output").textContent = JSON.stringify(
    parseResult,
    null,
    2
  );
});
