import init, {do_compile} from "../zeblang_wasm/pkg/zeblang_wasm.js";


function getText() {
  return document.getElementById("codeEditor").value
}

function hw() {
 init().then(() => {
   do_compile(getText());
 });
}

document.getElementById("runBtn").addEventListener("click", hw)
