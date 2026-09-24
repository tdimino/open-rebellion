// Implementation smoke only; this does not replace original-executable/Astra acceptance.
// Serve a production WASM build with contributor-owned runtime assets first.
import { chromium } from 'playwright-core';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { PNG } from 'pngjs';

const url = process.env.OPTIONS_TEST_URL ?? 'http://127.0.0.1:8769';
const output = process.env.OPTIONS_TEST_OUTPUT ?? '/tmp/open-rebellion-options-smoke';
await fs.mkdir(output, { recursive: true });
const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH,
  headless: true,
  args: ['--mute-audio', '--no-sandbox', '--use-angle=swiftshader', '--enable-unsafe-swiftshader'],
});
const results = [];
try {
  for (const faction of ['alliance', 'empire']) {
    const context = await browser.newContext({ viewport: { width: 1280, height: 960 } });
    const page = await context.newPage();
    await page.addInitScript(() => localStorage.setItem('rebellion_save_v13_9', 'preserve hidden slot'));

    const consoleLog = [], errors = [], network = [];
    page.on('console', message => consoleLog.push(message.text()));
    page.on('pageerror', error => errors.push(String(error)));
    page.on('response', response => network.push({ url: response.url(), status: response.status() }));
    const screenshot = name => page.screenshot({ path: path.join(output, `${faction}-${name}.png`) });
    const click = async (x, y) => { await page.mouse.click(x, y, { delay: 150 }); await page.waitForTimeout(350); };
    const deleteButton = async () => {
      // Locate the existing red Delete label; six-row panel height depends on occupancy.
      const png = PNG.sync.read(await page.screenshot());
      let xSum = 0, ySum = 0, count = 0;
      for (let y = 340; y < 490; y++) for (let x = 450; x < 600; x++) {
        const index = (y * png.width + x) * 4;
        const [r, g, b] = png.data.subarray(index, index + 3);
        if (r > 130 && g < 130 && b < 140 && r > 1.8 * g) { xSum += x; ySum += y; count++; }
      }
      assert.ok(count > 5, 'save-management panel renders its Delete label');
      await click(xSum / count, ySum / count);
    };
    const key = async value => { await page.keyboard.press(value, { delay: 100 }); await page.waitForTimeout(350); };
    const storage = () => page.evaluate(() => Object.fromEntries(Object.entries(localStorage)));
    try {
      await page.goto(url);
      await page.waitForTimeout(15000);
      await click(850, 500); // main-menu CD-ROM: original command 0x68
      await screenshot('main-options');
      await click(720, 185); // keep in-game music muted as well as Chromium
      await key('Escape');
      await click(faction === 'alliance' ? 945 : 360, 665);
      await page.waitForTimeout(1500);
      await key('F1');
      await screenshot('campaign-options');
      await click(350, 180);
      await page.keyboard.type(`${faction} options smoke`);
      await click(110, 181);
      const saved = await storage();
      assert.ok(saved.rebellion_save_v13_0, 'save writes the existing browser payload');
      assert.match(saved.rebellion_meta_v13_0, /options smoke/);
      await click(110, 181);
      await screenshot('overwrite-confirmation');
      await click(742, 602); // native No button
      assert.deepEqual(await storage(), saved, 'cancelled overwrite preserves storage');
      await click(610, 181);
      await screenshot('load-confirmation');
      await key('Escape');
      assert.deepEqual(await storage(), saved, 'cancelled load preserves storage');
      await click(610, 181);
      await click(560, 602); // native Yes button; restore campaign
      await key('F1');
      await key('F9');
      await screenshot('legacy-save-delete');
      await deleteButton();
      await screenshot('delete-confirmation');
      await click(742, 602);
      assert.deepEqual(await storage(), saved, 'cancelled deletion preserves storage');
      await key('F9');
      await deleteButton();
      await click(560, 602);
      assert.equal((await storage()).rebellion_save_v13_0, undefined, 'confirmed deletion removes payload');
      assert.equal((await storage()).rebellion_meta_v13_0, undefined, 'confirmed deletion removes metadata');
      await key('F8');
      await screenshot('legacy-load');
      await key('Escape');
      await click(195, 805); // restart confirmation
      await screenshot('restart-confirmation');
      await key('Escape');
      await key('Escape'); // return to the original campaign
      await screenshot('returned-campaign');
      // Prove that return still reaches a live campaign by re-entering F1.
      const priorEntries = consoleLog.filter(line => line.includes('destination=game_options status=opened_original')).length;
      await key('F1');
      assert.equal(consoleLog.filter(line => line.includes('destination=game_options status=opened_original')).length, priorEntries + 1);
      await screenshot('reentered-options');
      await page.evaluate(() => {
        localStorage.setItem('rebellion_save_v13_1', 'corrupt fixture');
        localStorage.setItem('rebellion_meta_v13_1', '{');
      });
      await key('Escape');
      await key('F1');
      const corrupt = await storage();
      await click(610, 265);
      await click(560, 602);
      await screenshot('corrupt-load-error');
      assert.deepEqual(await storage(), corrupt, 'failed load preserves corrupt bytes');
      await click(110, 265);
      await screenshot('corrupt-overwrite-confirmation');
      await click(742, 602);
      assert.deepEqual(await storage(), corrupt, 'corrupt-slot overwrite cancellation preserves bytes');
      assert.ok(consoleLog.some(line => line.includes('destination=game_options status=opened_original')));
      assert.equal((await storage()).rebellion_save_v13_9, 'preserve hidden slot', 'hidden older save data remains untouched');
      assert.equal(errors.length, 0, errors.join('\n'));
      results.push({ faction, status: 'pass', assertions: ['save', 'overwrite cancel', 'load cancel', 'load accept', 'delete cancel', 'delete accept', 'return/re-entry', 'corrupt load/overwrite cancel', 'no page errors'] });
    } catch (error) {
      results.push({ faction, status: 'fail', error: String(error) });
      throw error;
    } finally {
      await fs.writeFile(path.join(output, `${faction}-logs.json`), JSON.stringify({ consoleLog, errors, network }, null, 2));
      await context.close();
    }
  }
} finally {
  await browser.close();
  await fs.writeFile(path.join(output, 'results.json'), JSON.stringify(results, null, 2));
}
console.log(JSON.stringify(results));
