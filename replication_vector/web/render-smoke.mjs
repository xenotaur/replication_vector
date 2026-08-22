import { spawn, execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "../..");
const OUT = resolve(HERE, "smoke-out");
const SCENES = new Set(["first-scene", "replay", "sandbox"]);
const SCENE = SCENES.has(process.env.SMOKE_SCENE) ? process.env.SMOKE_SCENE : "first-scene";
const ARTIFACT = resolve(OUT, `${SCENE}.png`);
const METADATA = resolve(OUT, `${SCENE}.json`);
const PORT = process.env.SMOKE_PORT || "5173";
const BASE = (process.env.SMOKE_URL || `http://127.0.0.1:${PORT}`).replace(/\/$/, "");
const TARGET_URL = SCENE === "first-scene" ? BASE : `${BASE}/?scene=${SCENE}`;
const VIEWPORT = { width: 1024, height: 768 };
const READY_TEXTS = {
  "first-scene": "Velumin rendered 4 scene commands",
  replay: "Velumin rendered 4 replay commands",
  sandbox: "Velumin rendered 3 sandbox commands",
};
const READY_TEXT = READY_TEXTS[SCENE];
const READY_TIMEOUT_MS = 15000;

const LAUNCH_ARGS = [
  "--enable-unsafe-webgpu",
  "--enable-features=Vulkan,WebGPU",
  "--use-angle=metal",
  "--ignore-gpu-blocklist",
];

function commandOutput(command, args, fallback) {
  try {
    return execFileSync(command, args, {
      cwd: ROOT,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch {
    return fallback;
  }
}

function startServer() {
  if (process.env.SMOKE_URL) {
    return null;
  }

  const server = spawn("npm", ["run", "dev", "--", "--port", PORT, "--strictPort"], {
    cwd: HERE,
    stdio: ["ignore", "pipe", "pipe"],
  });

  server.stdout.on("data", (chunk) => process.stdout.write(chunk));
  server.stderr.on("data", (chunk) => process.stderr.write(chunk));

  return server;
}

async function waitForServer(page) {
  const started = Date.now();
  let lastError = null;

  while (Date.now() - started < READY_TIMEOUT_MS) {
    try {
      await page.goto(TARGET_URL, { waitUntil: "domcontentloaded", timeout: 2000 });
      return;
    } catch (error) {
      lastError = error;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 250));
    }
  }

  throw new Error(`Vite harness did not become reachable at ${BASE}: ${lastError}`);
}

async function hasWebGpu(browser) {
  const page = await browser.newPage();
  try {
    await waitForServer(page);
    return await page.evaluate(async () => {
      if (!navigator.gpu) return false;
      try {
        return Boolean(await navigator.gpu.requestAdapter());
      } catch {
        return false;
      }
    });
  } finally {
    await page.close();
  }
}

async function capture(browser) {
  const page = await browser.newPage({ viewport: VIEWPORT });
  try {
    await waitForServer(page);
    await page.waitForFunction(
      (readyText) => {
        const status = document.querySelector("#status")?.textContent || "";
        return status === readyText || /WebGPU|GPU|adapter|renderer|failed|error/i.test(status);
      },
      READY_TEXT,
      { timeout: READY_TIMEOUT_MS },
    );

    const status = await page.locator("#status").textContent();
    if (status !== READY_TEXT) {
      throw new Error(`render status did not reach "${READY_TEXT}"; saw "${status}"`);
    }

    if (SCENE === "sandbox") {
      await driveSandbox(page);
    }

    await mkdir(OUT, { recursive: true });
    await page.locator("#scene").screenshot({ path: ARTIFACT });

    const render = await page.evaluate(() => window.replicationVectorLastRender);
    if (!render) {
      throw new Error("browser did not expose replicationVectorLastRender metadata");
    }

    const metadata = {
      artifact: ARTIFACT,
      commandCount: render.commandCount,
      input: render.input,
      replay: render.replay,
      scene: render.scene,
      state: render.state,
      status,
      tuning: render.tuning,
      url: TARGET_URL,
      viewport: VIEWPORT,
      veluminCommit: commandOutput("git", ["-C", resolve(ROOT, ".deps/velumin"), "rev-parse", "--short", "HEAD"], "unknown"),
      capturedAt: new Date().toISOString(),
    };
    await writeFile(METADATA, `${JSON.stringify(metadata, null, 2)}\n`);

    console.log(`Render smoke screenshot: ${ARTIFACT}`);
    console.log(`Render smoke metadata:   ${METADATA}`);
  } finally {
    await page.close();
  }
}

async function setRange(page, selector, value) {
  await page.locator(selector).evaluate(
    (input, nextValue) => {
      input.value = nextValue;
      input.dispatchEvent(new Event("input", { bubbles: true }));
    },
    value,
  );
}

async function driveSandbox(page) {
  const before = await page.evaluate(() => window.replicationVectorLastRender?.state);
  if (!before) {
    throw new Error("sandbox did not expose initial state metadata");
  }

  await setRange(page, "#weight", "0.25");
  await setRange(page, "#inertia", "0.75");
  await setRange(page, "#responsiveness", "0.9");
  await page.keyboard.down("ArrowUp");
  await page.keyboard.down("ArrowRight");

  await page.waitForFunction(
    (initial) => {
      const render = window.replicationVectorLastRender;
      if (!render || render.scene !== "sandbox") return false;
      const moved = Math.abs(render.state.position.x - initial.position.x) > 0.0001;
      const turned = Math.abs(render.state.headingRadians - initial.headingRadians) > 0.0001;
      const tuned = render.tuning.weight === 0.25 && render.tuning.inertia === 0.75 && render.tuning.responsiveness === 0.9;
      const keyed = render.input.thrust === 1 && render.input.turn === 1;
      const config = render.state.config;
      const mapped =
        Math.abs(config.thrustAcceleration - 0.65) < 0.0001 &&
        Math.abs(config.turnAcceleration - 2.8) < 0.0001 &&
        Math.abs(config.linearDrag - 0.125) < 0.0001 &&
        Math.abs(config.angularDrag - 1.05) < 0.0001 &&
        Math.abs(config.maxSpeed - 1.75) < 0.0001 &&
        Math.abs(config.maxAngularSpeed - 4.27256632) < 0.0001;
      return moved && turned && tuned && keyed && mapped;
    },
    before,
    { timeout: READY_TIMEOUT_MS },
  );

  await page.evaluate(() => window.dispatchEvent(new Event("blur")));
  await page.keyboard.up("ArrowUp");
  await page.keyboard.up("ArrowRight");
  await page.waitForFunction(() => {
    const render = window.replicationVectorLastRender;
    return render?.input?.thrust === 0 && render.input.turn === 0;
  });

  await page.locator("#responsiveness").focus();
  await page.keyboard.press("ArrowLeft");
  const responsiveness = await page.locator("#responsiveness").inputValue();
  if (responsiveness !== "0.89") {
    throw new Error(`focused range input did not handle ArrowLeft; saw responsiveness=${responsiveness}`);
  }

  await page.evaluate(() => {
    window.replicationVectorTestSetState?.({
      position: { x: 1.149, y: 0 },
      velocity: { x: 1.75, y: 0 },
      headingRadians: 0,
      angularVelocityRadiansPerSecond: 0,
      config: null,
    });
    document.activeElement?.blur();
  });
  await page.keyboard.down("ArrowUp");
  await page.waitForFunction(() => {
    const render = window.replicationVectorLastRender;
    return render?.state?.position?.x < -1.0;
  });
}

async function launchBrowser() {
  try {
    return await chromium.launch({ headless: true, args: LAUNCH_ARGS });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (/Executable doesn't exist|Please run the following command to download new browsers|playwright install chromium/i.test(message)) {
      console.log(
        "SKIP: Playwright Chromium is not installed; run " +
          "`npx playwright install chromium` from replication_vector/web to enable render smoke capture.",
      );
      return null;
    }

    throw error;
  }
}

async function main() {
  const server = startServer();

  const browser = await launchBrowser();
  if (!browser) {
    if (server) {
      server.kill("SIGTERM");
    }
    return 0;
  }

  try {
    if (!(await hasWebGpu(browser))) {
      console.log(
        "SKIP: no WebGPU adapter available in this browser environment; " +
          "render smoke capture requires a WebGPU-capable Chromium.",
      );
      return 0;
    }

    await capture(browser);
    return 0;
  } finally {
    await browser.close();
    if (server) {
      server.kill("SIGTERM");
    }
  }
}

main()
  .then((code) => process.exit(code))
  .catch((error) => {
    console.error("render smoke failed:", error);
    process.exit(2);
  });
