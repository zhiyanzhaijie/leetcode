import { ChevronDown, ListTree } from 'lucide-react';
import type { MouseEvent } from 'react';
import type { MarkdownOutlineItem } from '../../lib/markdownOutline';

type Props = {
  items: MarkdownOutlineItem[];
};

function OutlineLinks({ items }: Props) {
  function closeMobileToc(event: MouseEvent<HTMLAnchorElement>) {
    const details = event.currentTarget.closest('details');
    if (details instanceof HTMLDetailsElement) details.open = false;
  }

  return (
    <ol>
      {items.map((item) => (
        <li key={`${item.line}-${item.id}`} data-depth={item.depth}>
          <a href={`#${item.id}`} onClick={closeMobileToc}>{item.text}</a>
        </li>
      ))}
    </ol>
  );
}

export default function SolutionTableOfContents({ items }: Props) {
  if (items.length === 0) return null;

  return (
    <>
      <details className="solution-toc solution-toc--mobile">
        <summary>
          <ListTree aria-hidden="true" size={15} strokeWidth={1.5} />
          On this page
          <span className="solution-toc__count">{items.length}</span>
          <ChevronDown className="solution-toc__chevron" aria-hidden="true" size={14} strokeWidth={1.5} />
        </summary>
        <nav aria-label="Table of contents">
          <OutlineLinks items={items} />
        </nav>
      </details>

      <aside className="solution-toc solution-toc--desktop">
        <div className="solution-toc__label">
          <ListTree aria-hidden="true" size={15} strokeWidth={1.5} />
          On this page
        </div>
        <nav aria-label="Table of contents">
          <OutlineLinks items={items} />
        </nav>
      </aside>
    </>
  );
}
