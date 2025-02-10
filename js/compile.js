import init, {interpret_zeblang} from "../zeblang_wasm/pkg/zeblang.js";
import {zext_dump_text} from "../zext/zext.js";

// add waiting page for initialisation
await init();

function getText() {
  return zext_dump_text();
}

function hw() {
   var stdout = document.getElementById("stdout");
   var stdexit = document.getElementById("stdexit")
   stdout.innerHTML = "";
   stdexit.innerHTML = "";
   stdexit.innerHTML += "exit: ";
   console.log(getText());
   stdexit.innerHTML += interpret_zeblang(getText());
}

document.getElementById("runBtn").addEventListener("click", hw)
