// Implementation smoke only; this does not replace original-executable/independent browser acceptance.
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
    page.setDefaultTimeout(30000);
    if (process.env.OPTIONS_TEST_MISSING_DIALOG === '1') {
      await page.route('**/data/runtime.orpk', async route => {
        const response = await route.fetch();
        const pack = await response.body();
        assert.equal(pack.subarray(0, 4).toString(), 'ORPK');
        const entries = [];
        let cursor = 12, removed = 0;
        for (let i = 0; i < pack.readUInt32LE(8); i++) {
          const start = cursor, kind = pack[cursor], keyLength = pack.readUInt16LE(cursor + 1), size = pack.readUInt32LE(cursor + 3);
          const key = pack.subarray(cursor + 7, cursor + 7 + keyLength).toString();
          cursor += 7 + keyLength + size;
          if (kind === 1 && key === 'rebdlog-dll/10623') removed++;
          else entries.push(pack.subarray(start, cursor));
        }
        assert.equal(removed, 1, 'fallback fixture removes precisely the confirmation background');
        const header = Buffer.from(pack.subarray(0, 12));
        header.writeUInt32LE(entries.length, 8);
        await route.fulfill({ response, body: Buffer.concat([header, ...entries]) });
      });
    }

    const consoleLog = [], errors = [], network = [];
    page.on('console', message => consoleLog.push(message.text()));
    page.on('pageerror', error => errors.push(String(error)));
    page.on('response', response => network.push({ url: response.url(), status: response.status() }));
    const screenshot = name => page.screenshot({ path: path.join(output, `${faction}-${name}.png`) });
    const click = async (x, y) => { await page.mouse.click(x, y, { delay: 300 }); await page.waitForTimeout(350); };
    const deleteButton = async () => {
      // Locate the existing red Delete label; panel height depends on occupancy.
      const png = PNG.sync.read(await page.screenshot());
      let xSum = 0, ySum = 0, count = 0;
      for (let y = 340; y < 615; y++) for (let x = 450; x < 600; x++) {
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
      await key('Control+a');
      await page.keyboard.type(`${faction} options smoke`);
      await click(110, 181);
      const saved = await storage();
      const saveKey = Object.keys(saved).find(key => /^rebellion_save_v\d+_0$/.test(key));
      assert.ok(saveKey, 'save writes the current-version browser payload');
      const saveVersion = Number(saveKey.match(/^rebellion_save_v(\d+)_0$/)?.[1]);
      assert.ok(Number.isInteger(saveVersion), 'save key exposes its current format version');
      const metaKey = `rebellion_meta_v${saveVersion}_0`;
      assert.match(saved[metaKey], /options smoke/);
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
      const saveLine = consoleLog.find(line => line.includes('save_state_fingerprint slot=0'));
      const fingerprint = saveLine?.match(/fingerprint=(\S+)/)?.[1];
      assert.ok(fingerprint, 'save reports a state fingerprint');
      assert.ok(consoleLog.some(line => line.includes('load_state_fingerprint slot=0') && line.includes(`fingerprint=${fingerprint}`)), 'accepted load reports the saved fingerprint');
      assert.ok(consoleLog.some(line => line.includes('[campaign] restored slot=0 mode=Galaxy') && line.includes(`fingerprint=${fingerprint}`) && line.includes(`faction=${faction === 'alliance' ? 'Alliance' : 'Empire'}`)), 'live restored campaign matches saved fingerprint and faction');
      await page.evaluate(({ saved, saveKey, metaKey, saveVersion }) => {
        localStorage.setItem(`rebellion_save_v${saveVersion}_9`, saved[saveKey]);
        localStorage.setItem(`rebellion_meta_v${saveVersion}_9`, saved[metaKey]);
      }, { saved, saveKey, metaKey, saveVersion });
      await key('F1');
      await key('F8');
      await page.mouse.move(530, 500);
      await page.mouse.wheel(0, 800);
      await page.waitForTimeout(500);
      await screenshot('slot-ten-load-panel');
      await click(510, 601); // last visible row after scrolling to the bottom
      await click(475, 651);
      await click(560, 602);
      assert.ok(consoleLog.some(line => line.includes('load_state_fingerprint slot=9') && line.includes(`fingerprint=${fingerprint}`)), 'slot 10 reports the existing save fingerprint');
      assert.ok(consoleLog.some(line => line.includes('[campaign] restored slot=9 mode=Galaxy') && line.includes(`fingerprint=${fingerprint}`)), 'slot 10 restores the live campaign');
      await key('F1');
      await key('F9');
      await page.mouse.move(530, 500);
      await page.mouse.wheel(0, 800);
      await page.waitForTimeout(500);
      await screenshot('slot-ten-delete-panel');
      await deleteButton();
      await click(560, 602);
      assert.deepEqual(await storage(), saved, 'deleting slot 10 preserves slot 1 and removes only its own payload and metadata');
      await key('F9');
      await page.mouse.move(530, 500);
      await page.mouse.wheel(0, -800);
      await page.waitForTimeout(500);
      await screenshot('legacy-save-delete');
      await deleteButton();
      await screenshot('delete-confirmation');
      await click(742, 602);
      assert.deepEqual(await storage(), saved, 'cancelled deletion preserves storage');
      await key('F9');
      await deleteButton();
      await click(560, 602);
      assert.equal((await storage())[saveKey], undefined, 'confirmed deletion removes payload');
      assert.equal((await storage())[metaKey], undefined, 'confirmed deletion removes metadata');
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
      await page.evaluate(saveVersion => {
        localStorage.setItem(`rebellion_save_v${saveVersion}_1`, 'corrupt fixture');
        localStorage.setItem(`rebellion_meta_v${saveVersion}_1`, '{');
      }, saveVersion);
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
      await click(544, 805); // Exit from the original window
      await screenshot('exit-confirmation');
      await click(560, 602);
      await page.getByRole('heading', { name: 'Game closed' }).waitFor();
      assert.ok(consoleLog.some(line => line.includes('[quit] audio_stopped=true cleanup=complete')));
      await screenshot('browser-exit');
      await page.getByRole('button', { name: 'Restart game' }).click();
      await page.waitForTimeout(15000);
      assert.equal(await page.getByRole('heading', { name: 'Game closed' }).count(), 0);
      await screenshot('browser-restarted');
      assert.equal(errors.length, 0, errors.join('\n'));
      results.push({ faction, status: 'pass', fingerprint, restoredLoads: consoleLog.filter(line => line.includes('[campaign] restored')), assertions: ['current-version save', 'overwrite cancel', 'load cancel', 'matching load and restored Galaxy fingerprint', 'slot 10 load/delete', 'delete cancel', 'delete accept', 'return/re-entry', 'corrupt load/overwrite cancel', 'browser exit/restart', 'no page errors'] });
    } catch (error) {
      await screenshot('failure').catch(() => {});
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
