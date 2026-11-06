/*
import { prepare, layout } from "https://esm.sh/@chenglou/pretext";

const canvas = document.getElementById("bg");
//const ctx = canvas.getContext("2d");

function resize() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}
resize();

const TEXT = "Learning Rust • Async • Concurrency • ";
const FONT = "20px monospace";

const handle = await prepare(TEXT.repeat(10), FONT);
const ctx = handle.getContext("2d");

let time = 0;
let mouseX = 0;
let mouseY = 0;

window.addEventListener("mousemove", (e) => {
    mouseX = e.clientX;
    mouseY = e.clientY;
});
function draw() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.font = FONT;
    ctx.fillStyle = "white";

    const result = layout(handle, canvas.width, 30);

    // 🔥 DEBUG FIRST
    console.log(result);

    // ❗ SAFETY CHECK
    if (!result || !result.glyphs) return;

    const glyphs = result.glyphs;

    for (let i = 0; i < glyphs.length; i++) {
        const g = glyphs[i];

        // 👉 REAL positions
        let x = g.x;
        let y = g.y;
        const dx = x - mouseX;
        const dy = y - mouseY;
        const dist = Math.sqrt(dx * dx + dy * dy);

        y += Math.sin(time * 0.1 + dist * 0.05) * 10;
        // ✨ animation
        y += Math.sin(time * 0.05 + x * 0.01) * 5;

        ctx.fillText(g.char, x, y);
    }

    time++;
    requestAnimationFrame(draw);
}

draw();*/
