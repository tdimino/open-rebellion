import { useCallback, useState } from 'react';

/**
 * A reusable multi-select hook supporting standard selection idioms:
 *
 *   - click           replace selection with one item
 *   - ctrl+click      toggle one item in selection
 *   - shift+click     extend selection to range from anchor
 *
 * The hook is keyed by item ID; the consumer maintains the ordered list.
 */
export function useSelection<TId extends number | string>() {
  const [selected, setSelected] = useState<Set<TId>>(new Set());
  const [anchor, setAnchor] = useState<TId | null>(null);

  const select = useCallback((id: TId) => {
    setSelected(new Set([id]));
    setAnchor(id);
  }, []);

  const toggle = useCallback((id: TId) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
    setAnchor(id);
  }, []);

  const selectRange = useCallback((id: TId, orderedIds: TId[]) => {
    if (anchor === null) {
      setSelected(new Set([id]));
      setAnchor(id);
      return;
    }
    const ia = orderedIds.indexOf(anchor);
    const ib = orderedIds.indexOf(id);
    if (ia < 0 || ib < 0) return;
    const [lo, hi] = ia < ib ? [ia, ib] : [ib, ia];
    setSelected(new Set(orderedIds.slice(lo, hi + 1)));
  }, [anchor]);

  const selectAll = useCallback((ids: TId[]) => {
    setSelected(new Set(ids));
    if (ids.length > 0) setAnchor(ids[ids.length - 1]);
  }, []);

  const clear = useCallback(() => {
    setSelected(new Set());
    setAnchor(null);
  }, []);

  /** Handle a click event in a row — picks the right behavior based on modifiers. */
  const handleClick = useCallback(
    (id: TId, e: React.MouseEvent, orderedIds: TId[]) => {
      if (e.shiftKey) {
        selectRange(id, orderedIds);
      } else if (e.ctrlKey || e.metaKey) {
        toggle(id);
      } else {
        select(id);
      }
    },
    [select, selectRange, toggle],
  );

  return {
    selected,
    anchor,
    select,
    toggle,
    selectRange,
    selectAll,
    clear,
    handleClick,
    isSelected: (id: TId) => selected.has(id),
    size: selected.size,
    toArray: () => [...selected],
  };
}
