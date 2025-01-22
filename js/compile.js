import init, {do_compile} from "../zeblang_wasm/pkg/zeblang_wasm.js";

function getText() {
  return document.getElementById("codeEditor").value
}

function hw() {
 init().then(() => {
   var stdout = document.getElementById("stdout");
   var stdexit = document.getElementById("stdexit")
   stdout.innerHTML = "";
   stdexit.innerHTML = "";
   stdexit.innerHTML += "exit: ";
   stdexit.innerHTML += do_compile(getText());
 });
}

document.getElementById("runBtn").addEventListener("click", hw)
