import { Emulator } from "space-invaders-wasm";
import { memory, emulator_emulate_next } from "space-invaders-wasm/space_invaders_wasm_bg";
import * as sounds from "./modules/sounds.js";

const HEIGHT = 256;
const WIDTH = 224;

var emu;

const read_and_load = (e) => {
    let files = e.target.files;
    if (files.length != 1) {
        alert("Please select merged single ROM file (8 KiB)");
        return;
    }
    let reader = new FileReader();
    reader.onloadend = (e) => {
        emu = Emulator.new(new Uint8Array(e.target.result));
        vram = new Uint8Array(memory.buffer, emu.get_vram(), 0x1BFF);
        requestAnimationFrame(renderLoop);
    };
    reader.readAsArrayBuffer(files[0]);
};

let file = document.getElementById("file-input").files[0];
if(file != null){
    let temp = {target: document.getElementById("file-input")};
    read_and_load(temp);
}

document.getElementById("file-input").addEventListener("change",
    read_and_load,
    false
);

//const emu = Emulator.new();
//const vram = new Uint8Array(memory.buffer, emu.get_vram(), 0x1BFF);
var vram;
const canvas = document.getElementById("canvas");
canvas.height = HEIGHT;
canvas.width = WIDTH;
const ctx = canvas.getContext('2d', { alpha: false });
const canvasData = ctx.getImageData(0, 0, WIDTH, HEIGHT);
const scaleX = window.innerWidth / canvas.width;
const scaleY = window.innerHeight / canvas.height;
const scaleToFit = Math.min(scaleX, scaleY);
canvas.style.transformOrigin = '0.5 0.5';
canvas.style.transform = 'scale(' + Math.ceil(scaleToFit / 1.5) + ')';

const drawPixel = (x, y, r, g, b) => {
    let index = (x + y * WIDTH) * 4;
    canvasData.data[index + 0] = r;
    canvasData.data[index + 1] = g;
    canvasData.data[index + 2] = b;
}
const renderToImage = () => {
    let x = 0;
    let y = 255;
    for (let i = 0; i < 0x1BFF; i++) {
        let byte = vram[i];
        for (let mask = 0; mask < 8; mask++) {
            let bit = byte & (1 << mask);
            if (bit > 0) {
                drawPixel(x, y, 255, 255, 255);
            } else {
                drawPixel(x, y, 0, 0, 0);
            }

            if (y === 0) {
                x += 1;
                y = 255;
            } else {
                y -= 1;
            }
        }
    }
}



var flipflop = false;
const renderLoop = () => {
    for (let index = 0; index < 2; index++) {
        let cycles_left = 33333 / 2;
        while (cycles_left > 0) {
            let ticks = emu.emulate_next();
            cycles_left -= ticks;
        }
        if (emu.is_int_enabled()) {
            if (flipflop) {
                emu.interrupt(0x10);
            } else {
                emu.interrupt(0x8);
            }
            flipflop = !flipflop;
        }
    }

    renderToImage();
    ctx.putImageData(canvasData, 0, 0);

    requestAnimationFrame(renderLoop);
};
//requestAnimationFrame(renderLoop);

const move_left = (is_down) => {
    emu.move_left(is_down);
}
const move_right = (is_down) => {
    emu.move_right(is_down);
}
const shot = (is_down) => {
    emu.shot(is_down);
}
const insert_coin = (is_down) => {
    emu.insert_coin(is_down);
}
const one_player_start = (is_down) => {
    emu.player1_start(is_down);
}
const two_player_start = (is_down) => {
    emu.player2_start(is_down);
}

var dict = {
    65: move_left,
    68: move_right,
    32: shot,
    67: insert_coin,
    49: one_player_start,
    50: two_player_start
};
document.getElementsByTagName("body")[0].addEventListener("keydown", function (e) {
    let f = dict[e.which];
    if (f != null) {
        f(true);
    }
});
document.getElementsByTagName("body")[0].addEventListener("keyup", function (e) {
    let f = dict[e.which];
    if (f != null) {
        f(false);
    }
});



