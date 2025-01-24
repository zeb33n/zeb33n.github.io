import init, {interpret_zeblang} from "../zeblang_wasm/pkg/zeblang.js";

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
   stdexit.innerHTML += interpret_zeblang(getText());
 });
}

document.getElementById("runBtn").addEventListener("click", hw)
