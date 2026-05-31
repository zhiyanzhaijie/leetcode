import { useDeferredValue, useEffect, useMemo, useState } from 'react';
import MarkdownArticle from './MarkdownArticle';
import SolutionDetailHeader from './SolutionDetailHeader';
import type { Solution, SolutionMeta } from '../../lib/solutions';
import { fetchSolutionById, fetchSolutionMetas } from '../../lib/solutions';

const PAGE_SIZE = 8;
const BASE_PATH = import.meta.env.BASE_URL.replace(/\/$/, '');

type LoadState = 'idle' | 'loading' | 'ready' | 'error';

function readRouteId() {
  if (typeof window === 'undefined') return null;
  const pathname = BASE_PATH && window.location.pathname.startsWith(BASE_PATH)
    ? window.location.pathname.slice(BASE_PATH.length) || '/'
    : window.location.pathname;
  const match = pathname.match(/^\/(\d+)\/?$/);
  return match ? Number(match[1]) : null;
}
function routeHref(id: number | null) {
  return id === null ? `${BASE_PATH || ''}/` : `${BASE_PATH || ''}/${id}`;
}

function updateBrowserRoute(id: number | null) {
  window.history.pushState({}, '', routeHref(id));
  window.dispatchEvent(new CustomEvent('site:navigate'));
}

export default function HomePage() {
  const [items, setItems] = useState<SolutionMeta[]>([]);
  const [solution, setSolution] = useState<Solution | null>(null);
  const [routeId, setRouteId] = useState<number | null>(null);
  const [query, setQuery] = useState('');
  const deferredQuery = useDeferredValue(query);
  const [page, setPage] = useState(1);
  const [listState, setListState] = useState<LoadState>('idle');
  const [detailState, setDetailState] = useState<LoadState>('idle');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const syncRoute = () => setRouteId(readRouteId());
    syncRoute();
    window.addEventListener('popstate', syncRoute);
    window.addEventListener('site:navigate', syncRoute);
    return () => {
      window.removeEventListener('popstate', syncRoute);
      window.removeEventListener('site:navigate', syncRoute);
    };
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function loadMetas() {
      try {
        setListState('loading');
        setError(null);
        const metas = await fetchSolutionMetas();
        if (!cancelled) {
          setItems(metas);
          setListState('ready');
        }
      } catch (err) {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : 'Failed to load solutions.');
          setListState('error');
        }
      }
    }

    loadMetas();
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function loadSolution(id: number) {
      try {
        setDetailState('loading');
        setError(null);
        const nextSolution = await fetchSolutionById(id);
        if (!cancelled) {
          setSolution(nextSolution);
          setDetailState(nextSolution ? 'ready' : 'error');
          if (!nextSolution) setError(`Solution ${id} was not found on GitHub.`);
        }
      } catch (err) {
        if (!cancelled) {
          setSolution(null);
          setError(err instanceof Error ? err.message : 'Failed to load solution.');
          setDetailState('error');
        }
      }
    }

    if (routeId === null) {
      setSolution(null);
      setDetailState('idle');
      return;
    }

    loadSolution(routeId);
    return () => {
      cancelled = true;
    };
  }, [routeId]);

  const filteredItems = useMemo(() => {
    const q = deferredQuery.trim().toLowerCase();
    if (!q) return items;

    return items.filter((item) => {
      const haystack = `${item.id} ${item.title} ${item.difficulty} ${item.summary} ${item.tags.join(' ')}`.toLowerCase();
      return haystack.includes(q);
    });
  }, [items, deferredQuery]);

  const pageCount = Math.max(1, Math.ceil(filteredItems.length / PAGE_SIZE));
  const safePage = Math.min(page, pageCount);
  const pagedItems = filteredItems.slice((safePage - 1) * PAGE_SIZE, safePage * PAGE_SIZE);
  const currentIndex = items.findIndex((item) => item.id === routeId);
  const prev = currentIndex > 0 ? items[currentIndex - 1] : null;
  const next = currentIndex >= 0 && currentIndex < items.length - 1 ? items[currentIndex + 1] : null;

  function updateQuery(value: string) {
    setQuery(value);
    setPage(1);
  }

  function navigate(id: number | null) {
    updateBrowserRoute(id);
    window.scrollTo({ top: 0, behavior: 'auto' });
  }

  if (routeId !== null) {
    return (
      <div className="home-page">
        {detailState === 'loading' && <p className="empty-state">Loading solution from GitHub...</p>}
        {detailState === 'error' && <p className="empty-state">{error ?? 'Failed to load solution.'}</p>}
        {solution && (
          <>
            <SolutionDetailHeader current={solution} prev={prev} next={next} onNavigate={navigate} />
            <MarkdownArticle markdown={solution.markdown} assets={solution.assets} />
          </>
        )}
      </div>
    );
  }

  return (
    <div className="home-page">
      <section className="home-hero" aria-labelledby="site-title">
        <p className="eyebrow">leetcode notes</p>
        <h1 id="site-title">leetcode notes of @zhiyanzhaijie</h1>
        <p>Self solution point for thinking and re-thinking.</p>
      </section>

      <section className="solution-list" aria-labelledby="solution-list-title">
        <div className="section-line">
          <h2 id="solution-list-title">solutions</h2>
          <span>{filteredItems.length} / {items.length}</span>
        </div>
        <div className="solution-tools">
          <label htmlFor="solution-search">search problem</label>
          <input
            id="solution-search"
            value={query}
            onChange={(event) => updateQuery(event.target.value)}
            placeholder="id, title, difficulty, tag..."
            type="search"
          />
        </div>
        {listState === 'loading' && <p className="empty-state">Loading solutions from GitHub...</p>}
        {listState === 'error' && <p className="empty-state">{error ?? 'Failed to load solutions.'}</p>}
        {listState === 'ready' && (
          <>
            <ol>
              {pagedItems.map((item) => (
                <li key={item.id}>
                  <a
                    href={routeHref(item.id)}
                    onClick={(event) => {
                      event.preventDefault();
                      navigate(item.id);
                    }}
                  >
                    <span className="solution-list__id">{item.id}</span>
                    <span className="solution-list__body">
                      <strong>{item.title}</strong>
                      <small>{item.difficulty} · {item.tags.length > 0 ? item.tags.join(' / ') : 'algorithm'}</small>
                    </span>
                    <span className="solution-list__summary">{item.summary}</span>
                  </a>
                </li>
              ))}
            </ol>
            {pagedItems.length === 0 && <p className="empty-state">No solutions found.</p>}
            <div className="pagination" aria-label="Solution pages">
              <button type="button" onClick={() => setPage((current) => Math.max(1, current - 1))} disabled={safePage === 1}>
                previous
              </button>
              <span>{safePage} / {pageCount}</span>
              <button type="button" onClick={() => setPage((current) => Math.min(pageCount, current + 1))} disabled={safePage === pageCount}>
                next
              </button>
            </div>
          </>
        )}
      </section>
    </div>
  );
}
