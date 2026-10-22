const NATIVE_ORIGINS = {
  alliance: { x: 62, y: 50 },
  empire: { x: 125, y: 52 },
};
const INDEX_ORIGINS = {
  alliance: { x: 423, y: 147 },
  empire: { x: 426, y: 143 },
};

export function countMaskedRgbDifferences(actual, expected, opaqueMask) {
  if (actual.length !== expected.length || expected.length !== opaqueMask.length * 4) {
    throw new Error("RGB buffers and opacity mask must describe the same image");
  }
  let different = 0;
  for (let pixel = 0; pixel < opaqueMask.length; pixel += 1) {
    if (!opaqueMask[pixel]) continue;
    const offset = pixel * 4;
    if (actual[offset] !== expected[offset]
      || actual[offset + 1] !== expected[offset + 1]
      || actual[offset + 2] !== expected[offset + 2]) different += 1;
  }
  return different;
}

export function encyclopediaSurfaceGeometry(faction, viewport, probes = {}) {
  const nativeOrigin = NATIVE_ORIGINS[faction];
  if (!nativeOrigin) throw new Error(`Unknown Encyclopedia faction: ${faction}`);
  const indexOrigin = INDEX_ORIGINS[faction];

  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const origin = {
    x: Math.round((viewport.width - 640 * scale) / 2 + nativeOrigin.x * scale),
    y: Math.round((viewport.height - 480 * scale) / 2 + nativeOrigin.y * scale),
  };
  const artRect = {
    x: Math.round(origin.x + 12 * scale),
    y: Math.round(origin.y + 31 * scale),
    width: Math.round(400 * scale),
    height: Math.round(200 * scale),
  };
  const point = (base, probe = { x: 0, y: 0 }) => ({
    x: origin.x + (base.x + probe.x) * scale,
    y: origin.y + (base.y + probe.y) * scale,
  });
  const disabledBackRect = {
    x: Math.round(origin.x + 28 * scale),
    y: Math.round(origin.y + 14 * scale),
    width: Math.round(21 * scale),
    height: Math.round(17 * scale),
  };
  return {
    scale,
    origin,
    artRect,
    forwardPoint: point({ x: 380, y: 14 }, probes.forward),
    indexPoint: point(indexOrigin, probes.index),
    disabledBackRect,
  };
}
