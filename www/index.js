import * as sim from "lib-simulation-wasm";
import { Terminal } from "./app/terminal";
import { Viewport } from "./app/viewport";
/*
const terminal = new Terminal(
    document.getElementById("terminal-stdin"),
    document.getElementById("terminal-stdout"),
);

const viewport = new Viewport(
    document.getElementById("viewport"),
);

let simulation = new sim.Simulation(sim.Simulation.default_config()); // активная симуляция

let active = true; // симуляция активна или на паузе

const config = simulation.config();

terminal.println("Simulation of evolution, powered by neural network, genetic algorithm and high school math.");




*/






/*

document.getElementById('train').onclick = function() {
    console.log(simulation.train());
}

// отрисовка треугольника
CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
        this.beginPath();

        this.moveTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );

        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );

        this.lineTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );

        //this.stroke();

        this.fillStyle = 'rgb(247, 108, 108)';
        this.fill();
    };

// отрисовка круга
CanvasRenderingContext2D.prototype.drawCircle = 
function(x, y, radius) {
    this.beginPath();
    this.arc(x, y, radius, 0, 2.0 * Math.PI);
    this.fillStyle = 'rgb(36, 48, 94)';
    this.fill();
}

// функция для отрисовки птиц и пищи
function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
    simulation.step();

    const world = simulation.world();

    for (const food of world.foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    for (const animal of world.animals) {
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.015 * viewportWidth,
            animal.rotation,
        );
    }
    requestAnimationFrame(redraw); // зацикливание симуляции
}

redraw(); */