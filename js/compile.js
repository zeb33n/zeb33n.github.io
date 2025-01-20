import init, {do_compile} from "../zeblang_wasm/pkg/zeblang_wasm.js";
function hw() {
 init().then(() => {
   do_compile("exit 1 + 2\nout=1");
 });
}
document.getElementById("runBtn").addEventListener("click", hw)
