import type { SolutionMeta } from '../../lib/solutions';
const BASE_PATH = import.meta.env.BASE_URL.replace(/\/$/, '');

function routeHref(id: number | null) {
  return id === null ? `${BASE_PATH || ''}/` : `${BASE_PATH || ''}/${id}`;
}

type Props = {
  current?: SolutionMeta;
  prev?: SolutionMeta | null;
  next?: SolutionMeta | null;
  onNavigate?: (id: number | null) => void;
};

export default function SolutionDetailHeader({ current, prev, next, onNavigate }: Props) {
  return (
    <header className="solution-head">
      <a
        href={routeHref(null)}
        className="back-link"
        onClick={(event) => {
          if (!onNavigate) return;
          event.preventDefault();
          onNavigate(null);
        }}
      >
        cd ..
      </a>
      <p className="eyebrow">solution / {current?.difficulty ?? 'Unknown'}</p>
      <h1>
        <span>{current?.id}</span>
        {current?.title ?? 'Problem'}
      </h1>
      <p>{current?.summary}</p>
      <nav className="solution-pager" aria-label="Solution pagination">
        {prev && (
          <a
            href={routeHref(prev.id)}
            onClick={(event) => {
              if (!onNavigate) return;
              event.preventDefault();
              onNavigate(prev.id);
            }}
          >
            previous / {prev.id}
          </a>
        )}
        {next && (
          <a
            href={routeHref(next.id)}
            onClick={(event) => {
              if (!onNavigate) return;
              event.preventDefault();
              onNavigate(next.id);
            }}
          >
            next / {next.id}
          </a>
        )}
      </nav>
    </header>
  );
}
