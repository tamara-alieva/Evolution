import * as sim from "lib-simulation-wasm";
import { Terminal } from "./app/terminal";
import { Viewport } from "./app/viewport";

/* ---------- */

const terminal = new Terminal(
    document.getElementById("terminal-stdin"),
    document.getElementById("terminal-stdout"),
);

const viewport = new Viewport(
    document.getElementById("viewport"),
);

/**
 * Актуальная симуляция.
 *
 * @type {Simulation}
 */
let simulation = new sim.Simulation(sim.Simulation.default_config());

/**
 * 
 * Симуляция активна или на паузе
 *
 * @type {boolean}
 */
let active = true;

/* ---------- */

const config = simulation.config();

terminal.println("Симуляция эволюции с помощью нейронной сети и генетического алгоритма.");
terminal.println("");
terminal.println("---- О программе ----");
terminal.println("");
terminal.println("Каждый треугольник представляет птицу; у каждой птицы есть зрение, поле которого рисуется вокруг, и мозг, который определяет, куда и как быстро птица должна двигаться.");
terminal.println("");
terminal.println("Каждый круг - это пища, которую птицы должны искать и есть.");
terminal.println("");
terminal.println("Изначально мозг птицы случайно сгенерирован. После 2500 ходов (около 40 секунд) те птицы, которые съели больше всего еды, воспроизводят потомков для новой симуляции.");
terminal.println("");
terminal.println("Благодаря генетическому алгоритму каждое поколение становится лучше в поиске еды.");
terminal.println("");
terminal.println("Вы можете влиять на симуляцию, вводя команды в поле внизу этого блока:");
terminal.println("");
terminal.println("- p / pause");
terminal.println("  Приостанавливает (или возобновляет) симуляцию");
terminal.println("");
terminal.println(`- r / reset [animals=${config.world_animals}] [f=${config.world_foods}] [...]`);
terminal.println("  Перезапускает симуляцию с указанными ");
terminal.println("  опциональнальными параметрами:");
terminal.println("");
terminal.println(`  * a / animals (default=${config.world_animals})`);
terminal.println("    количество птиц");
terminal.println("");
terminal.println(`  * f / foods (default=${config.world_foods})`);
terminal.println("    количество пищи");
terminal.println("");
terminal.println(`  * n / neurons (default=${config.brain_neurons})`);
terminal.println("    количество нейронов мозга для каждого животного");
terminal.println("");
terminal.println(`  * p / photoreceptors (default=${config.eye_cells})`);
terminal.println("    количество клеток глаза каждого животного");
terminal.println("");
terminal.println("  Примеры:");
terminal.println("    reset animals=100 foods=100");
terminal.println("    r a=100 f=100");
terminal.println("    r p=3");
terminal.println("");
terminal.println("- (t)rain [кол-во поколений]");
terminal.println("  Ускоряет одно или несколько поколений, позволяя");
terminal.println("  быстро увидеть результат обучения.");
terminal.println("");
terminal.println("  Примеры:");
terminal.println("    train");
terminal.println("    t 5");
terminal.println("");
terminal.println("----");
terminal.scrollToTop();

/* ---------- */

terminal.onInput((input) => {
    terminal.println("");
    terminal.println("$ " + input);

    try {
        exec(input);
    } catch (err) {
        terminal.println(`  ^ ошибка: ${err}`);
    }
});

function exec(input) {
    if (input.includes("[") || input.includes("]")) {
        throw "Квадратные скобки использовались лишь для примера - Вам не нужно их писать, например: reset animals=100";
    }

    const [cmd, ...args] = input.split(" ");

    if (cmd === "p" || cmd === "pause") {
        execPause(args);
        return;
    }

    if (cmd === "r" || cmd === "reset") {
        execReset(args);
        return;
    }

    if (cmd === "t" || cmd === "train") {
        execTrain(args);
        return;
    }

    throw "Неизвестная команда";
}

function execPause(args) {
    if (args.length > 0) {
        throw "У этой команды нет параметров";
    }

    active = !active;
}

function execReset(args) {
    let config = sim.Simulation.default_config();

    for (const arg of args) {
        const [argName, argValue] = arg.split("=");

        if (argName.startsWith("i:")) {
            config[argName.slice(2)] = parseInt(argValue);
        } else if (argName.startsWith("f:")) {
            config[argName.slice(2)] = parseFloat(argValue);
        } else {
            switch (argName) {
                case "a":
                case "animals":
                    config.world_animals = parseInt(argValue);
                    break;

                case "f":
                case "foods":
                    config.world_foods = parseInt(argValue);
                    break;

                case "n":
                case "neurons":
                    config.brain_neurons = parseInt(argValue);
                    break;

                case "p":
                case "photoreceptors":
                    config.eye_cells = parseInt(argValue);
                    break;

                default:
                    throw `Неизвестный параметр: ${argName}`;
            }
        }
    }

    simulation = new sim.Simulation(config);
}

function execTrain(args) {
    if (args.length > 1) {
        throw "Эта команда принимает хотя бы один параметров";
    }

    const generations = args.length == 0 ? 1 : parseInt(args[0]);

    for (let i = 0; i < generations; i += 1) {
        if (i > 0) {
            terminal.println("");
        }

        const stats = simulation.train();
        terminal.println(stats);
    }
}

/* ---------- */

function redraw() {
    if (active) {
        const stats = simulation.step();

        if (stats) {
            terminal.println(stats);
        }
    }

    const config = simulation.config();
    const world = simulation.world();

    viewport.clear();

    for (const food of world.foods) {
        viewport.drawCircle( // пища
            food.x,
            food.y,
            (config.food_size / 2.0),
            'rgb(247, 108, 108)',
        );
    }

    for (const animal of world.animals) {
        viewport.drawTriangle( // птица
            animal.x,
            animal.y,
            config.food_size,
            animal.rotation,
            'rgb(36, 48, 94)',
        );

        const anglePerCell = config.eye_fov_angle / config.eye_cells;

        for (let cellId = 0; cellId < config.eye_cells; cellId += 1) {
            const angleFrom =
                  animal.rotation
                  - config.eye_fov_angle / 2.0
                  + cellId * anglePerCell
                  + Math.PI / 2.0;

            const angleTo = angleFrom + anglePerCell;
            const energy = animal.vision[cellId];

            viewport.drawArc( // окружность видимости пищи
                animal.x,
                animal.y,
                (config.food_size * 2.5),
                angleFrom,
                angleTo,
                `rgba(36, 48, 94, ${energy})`,
            );
        }
    }

    requestAnimationFrame(redraw);
}

redraw();
