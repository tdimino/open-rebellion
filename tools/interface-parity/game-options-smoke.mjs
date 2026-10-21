// Implementation smoke only; this does not replace original-executable/independent browser acceptance.
// Serve a production WASM build with contributor-owned runtime assets first.
import { chromium } from 'playwright-core';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';

const url = process.env.OPTIONS_TEST_URL ?? 'http://127.0.0.1:8769';
const output = process.env.OPTIONS_TEST_OUTPUT ?? '/tmp/open-rebellion-options-smoke';
await fs.mkdir(output, { recursive: true });
const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH,
  headless: true,
  args: ['--mute-audio', '--disable-audio-output', '--no-sandbox', '--use-angle=swiftshader', '--enable-unsafe-swiftshader'],
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
    const key = async value => { await page.keyboard.press(value, { delay: 100 }); await page.waitForTimeout(350); };
    const storage = () => page.evaluate(() => Object.fromEntries(Object.entries(localStorage)));
    // Wait until the console logs a line matching `pattern` (beyond the first
    // `after` matches). While waiting, a movie on screen is skipped by a left
    // press on it, as the player's click skips one (FUN_005f7240; the shell's
    // pointerdown handler, web/index.html). The press goes to the <video>
    // alone, so it can never land on the main menu behind it.
    const consoleLine = async (pattern, { after = 0, skipMovies = false, timeout = 240000 } = {}) => {
      const deadline = Date.now() + timeout;
      while (consoleLog.filter(line => pattern.test(line)).length <= after) {
        assert.ok(Date.now() < deadline, `no console line ${pattern} within ${timeout} ms`);
        if (skipMovies) {
          await page.evaluate(() => document.querySelector('video.movie')
            ?.dispatchEvent(new PointerEvent('pointerdown', { button: 0, bubbles: true })));
        }
        await page.waitForTimeout(250);
      }
    };
    // The opening movies end in the main menu (FUN_004138a0: 0x66 enters
    // state 3); a start's side movie ends in the galaxy (0x68/0x69, state 8).
    const MENU = /\[movie\] ended id=0x66 next=MainMenu/;
    const GALAXY = /\[movie\] ended id=0x6[89] next=Galaxy/;
    const INTRO = /\[cutscene\] opened path=assets\/references\/ref-videos\/000\.webm/;
    try {
      await page.goto(url);
      await consoleLine(MENU, { skipMovies: true });
      await page.waitForTimeout(350); // the menu's first frame after the movie
      await click(850, 500); // main-menu CD-ROM: original command 0x68
      await screenshot('main-options');
      await click(720, 185); // keep in-game music muted as well as Chromium
      await click(195, 805); // Restart (0x91): with no game, straight back to the shuttle cockpit
      await click(faction === 'alliance' ? 945 : 360, 665);
      await consoleLine(GALAXY, { skipMovies: true });
      await page.waitForTimeout(350); // the command center's first frame
      await key('F1');
      await screenshot('campaign-options');
      await click(350, 180);
      await key('Control+a');
      await page.keyboard.type(`${faction} options smoke`);
      await click(110, 181);
      // FUN_00408240: the save raises the 0x474 alert, "Selected Game Saved
      // Successfully", and flags row 1 for this opening of the window.
      await consoleLine(/save_state_fingerprint slot=0 /);
      await screenshot('saved-alert');
      await click(636, 600); // the alert's checkmark (0x1300, Yes)
      // FUN_00406b80 0x6f: a row this opening saved overwrites with no
      // confirmation; the alert's class takes Enter as its checkmark.
      await click(110, 181);
      await consoleLine(/save_state_fingerprint slot=0 /, { after: 1 });
      await screenshot('resaved-alert');
      await key('Enter');
      // A new opening clears the row flags (FUN_00406950).
      await click(366, 806); // Return icon (0x90)
      await key('F1');
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
      await key('Escape'); // the confirmation answers Escape with No (FUN_00416f30)
      assert.deepEqual(await storage(), saved, 'cancelled load preserves storage');
      await click(610, 181);
      await click(560, 602); // native Yes button; restore campaign
      const saveLine = consoleLog.find(line => line.includes('save_state_fingerprint slot=0'));
      const fingerprint = saveLine?.match(/fingerprint=(\S+)/)?.[1];
      assert.ok(fingerprint, 'save reports a state fingerprint');
      assert.ok(consoleLog.some(line => line.includes('load_state_fingerprint slot=0') && line.includes(`fingerprint=${fingerprint}`)), 'accepted load reports the saved fingerprint');
      assert.ok(consoleLog.some(line => line.includes('[campaign] restored slot=0 mode=Galaxy') && line.includes(`fingerprint=${fingerprint}`) && line.includes(`faction=${faction === 'alliance' ? 'Alliance' : 'Empire'}`)), 'live restored campaign matches saved fingerprint and faction');
      // Row 6, the window's last (y 0x123, FUN_00407180): an empty row saves
      // with no confirmation, reports its fingerprint, and loads back through
      // its own Load control. The original window has six rows and no delete
      // control (ghidra/notes/game-options.md), so nothing here deletes.
      await key('F1');
      await click(350, 602); // row 6 name field
      await key('Control+a');
      await page.keyboard.type(`${faction} slot six`);
      await click(110, 602); // row 6 Save
      await consoleLine(/save_state_fingerprint slot=5 /);
      const slotSix = consoleLog.find(line => line.includes('save_state_fingerprint slot=5 '))?.match(/fingerprint=(\S+)/)?.[1];
      assert.ok(slotSix, 'row 6 save reports a state fingerprint');
      const sixStored = await storage();
      assert.ok(sixStored[`rebellion_save_v${saveVersion}_5`], 'row 6 save writes its own payload');
      assert.match(sixStored[`rebellion_meta_v${saveVersion}_5`], /slot six/);
      assert.equal(sixStored[saveKey], saved[saveKey], 'row 6 save leaves row 1 untouched');
      await screenshot('slot-six-saved');
      await click(636, 600); // the saved alert's checkmark
      // Row 6 was saved in this opening, so Load asks nothing (FUN_00406b80
      // 0x79-0x7e skips 0x477 while a row flag is set).
      await click(610, 602); // row 6 Load
      await consoleLine(/\[campaign\] restored slot=5 mode=Galaxy/);
      assert.ok(consoleLog.some(line => line.includes('load_state_fingerprint slot=5 ') && line.includes(`fingerprint=${slotSix}`)), 'row 6 load reports the saved fingerprint');
      assert.ok(consoleLog.some(line => line.includes('[campaign] restored slot=5 mode=Galaxy') && line.includes(`fingerprint=${slotSix}`)), 'row 6 restores the live campaign');
      await key('F1'); // the options window again, for Restart
      await click(195, 805); // restart confirmation
      await screenshot('restart-confirmation');
      await click(742, 602); // native No button
      // The window itself handles no key (FUN_00406b80): Escape leaves it open.
      const menuReturns = consoleLog.filter(line => line.includes('returned_from_campaign')).length;
      await key('Escape');
      await screenshot('escape-keeps-options');
      await click(366, 806); // Return icon (0x90): back to the original campaign
      assert.equal(consoleLog.filter(line => line.includes('returned_from_campaign')).length, menuReturns, 'Escape and Return reach no main menu');
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
      await click(366, 806); // Return icon (0x90)
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
      const intros = consoleLog.filter(line => INTRO.test(line)).length;
      await page.getByRole('button', { name: 'Restart game' }).click();
      await consoleLine(INTRO, { after: intros }); // the restarted game opens its intro
      assert.equal(await page.getByRole('heading', { name: 'Game closed' }).count(), 0);
      await screenshot('browser-restarted');
      assert.equal(errors.length, 0, errors.join('\n'));
      results.push({ faction, status: 'pass', fingerprint, restoredLoads: consoleLog.filter(line => line.includes('[campaign] restored')), assertions: ['current-version save', 'overwrite cancel', 'load cancel', 'matching load and restored Galaxy fingerprint', 'row 6 save/load', 'saved alert, row-flag overwrite and load', 'Escape keeps the window, Return/re-entry', 'corrupt load/overwrite cancel', 'browser exit/restart', 'no page errors'] });
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
