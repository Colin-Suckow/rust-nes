import * as wasm from "nes";

async function fetchRom(path) {
    return fetch(path)
        .then(file => {
            return file
        })
}

function ready(fn) {
    if (document.readyState != 'loading') {
        fn();
    } else {
        document.addEventListener('DOMContentLoaded', fn)
    }
}

function draw_frame(ctx, buffer) {
    let canvasHeight = 240
    let canvasWidth = 256
    let imageData = ctx.getImageData(0, 0, canvasWidth, canvasHeight)
    let data = imageData.data
    for (var y = 0; y < canvasHeight; ++y) {
        for (var x = 0; x < canvasWidth; ++x) {
            var index = (y * canvasWidth + x)
            var offset = 4 * index
            let pixel = buffer[index]
            // light blue (#80d7ff)
            data[offset + 0] = (pixel >> 16) & 0xFF; // red
            data[offset + 1] = (pixel >> 8) & 0xFF; // green
            data[offset + 2] = (pixel) & 0xFF; // blue
            data[offset + 3] = 0xff; // alpha
        }
    }
    //Bit of a hack to scale the canvas
    ctx.putImageData(imageData, 0, 0);
    ctx.drawImage(ctx.canvas, 0, 0)
}

ready(async function () {

    let file = await fetchRom("dk.nes")
    let data_buffer = await file.arrayBuffer();
    console.log("File is " + data_buffer.byteLength + " bytes")

    let emu = wasm.Emulator.new(new Uint8Array(data_buffer))
    const ctx = document.getElementById("render_canvas").getContext('2d')
    ctx.webkitImageSmoothingEnabled = false;
    ctx.mozImageSmoothingEnabled = false;
    ctx.imageSmoothingEnabled = false;
    ctx.scale(2,2)
    let start_time;

    function frame(timestamp) {
        emu.run_frame()
        draw_frame(ctx, emu.buffer())
        window.requestAnimationFrame(frame)
    }

    window.requestAnimationFrame(frame)

})