import init, {greet} from "../zeblang_wasm/pkg/zeblang_wasm.js";


  function hw() {
    init().then(() => {
      greet("Zeb");
    });
  };
document.getElementById("runBtn").addEventListener("click", hw)
