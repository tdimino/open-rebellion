import { useEffect } from 'react';

type HotkeySpec = string; // "ctrl+a", "shift+enter", "escape", "delete"

interface ParsedHotkey {
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
  key: string;
}

function parse(spec: HotkeySpec): ParsedHotkey {
  const parts = spec.toLowerCase().split('+').map((p) => p.trim());
  return {
    ctrl: parts.includes('ctrl') || parts.includes('control'),
    shift: parts.includes('shift'),
    alt: parts.includes('alt'),
    meta: parts.includes('meta') || parts.includes('cmd'),
    key: parts[parts.length - 1],
  };
}

function matches(e: KeyboardEvent, hk: ParsedHotkey): boolean {
  return (
    (e.ctrlKey || e.metaKey) === (hk.ctrl || hk.meta) && // treat Cmd as Ctrl on Mac
    e.shiftKey === hk.shift &&
    e.altKey === hk.alt &&
    e.key.toLowerCase() === hk.key
  );
}

/**
 * Register a single hotkey. Listener is global (window-level).
 * Ignores events when an input/textarea/contenteditable is focused so
 * typing doesn't trigger app shortcuts.
 */
export function useHotkey(spec: HotkeySpec, handler: (e: KeyboardEvent) => void, deps: React.DependencyList = []) {
  useEffect(() => {
    const hk = parse(spec);
    const listener = (e: KeyboardEvent) => {
      const t = e.target as HTMLElement | null;
      if (t && isEditableTarget(t)) return;
      if (!matches(e, hk)) return;
      e.preventDefault();
      handler(e);
    };
    window.addEventListener('keydown', listener);
    return () => window.removeEventListener('keydown', listener);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [spec, ...deps]);
}

function isEditableTarget(t: HTMLElement): boolean {
  if (t.isContentEditable) return true;
  const tag = t.tagName.toLowerCase();
  return tag === 'input' || tag === 'textarea' || tag === 'select';
}
