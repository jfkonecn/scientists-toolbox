
import init, * as lib from "./lib/scientists_toolbox.js";

const app = Elm.Main.init({
    node: document.getElementById("app")
});


async function  main() {
    await init();
}
main();