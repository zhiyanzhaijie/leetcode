import { type ReactNode, useMemo } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import type { MarkdownOutlineItem } from '../../lib/markdownOutline';

type Props = {
  markdown: string;
  assets: Record<string, string>;
  outline: MarkdownOutlineItem[];
};

function normalizeMarkdown(markdown: string) {
  return markdown.replace(/\[img\]\(([^)]+)\)/g, '![img]($1)');
}

export default function MarkdownArticle({ markdown, assets, outline }: Props) {
  const normalizedMarkdown = useMemo(() => normalizeMarkdown(markdown), [markdown]);
  const headingIds = useMemo(() => new Map(outline.map((item) => [item.line, item.id])), [outline]);
  const markdownComponents = useMemo(
    () => ({
      h1() {
        return null;
      },
      h2({ node, children, ...props }: { node?: { position?: { start: { line: number } } }; children?: ReactNode }) {
        const id = node?.position ? headingIds.get(node.position.start.line) : undefined;
        return <h2 id={id} {...props}>{children}</h2>;
      },
      h3({ node, children, ...props }: { node?: { position?: { start: { line: number } } }; children?: ReactNode }) {
        const id = node?.position ? headingIds.get(node.position.start.line) : undefined;
        return <h3 id={id} {...props}>{children}</h3>;
      },
      pre({ children, ...props }: { children?: ReactNode }) {
        return (
          <div className="code-scroll">
            <pre {...props}>{children}</pre>
          </div>
        );
      },
      a({ href, children, ...props }: { href?: string; children?: ReactNode }) {
        const isExternal = href?.startsWith('http');
        return (
          <a href={href} target={isExternal ? '_blank' : undefined} rel={isExternal ? 'noreferrer' : undefined} {...props}>
            {children}
          </a>
        );
      },
      img({ src = '', alt = '', ...props }: { src?: string; alt?: string }) {
        const resolvedSrc = assets[src] ?? src;
        return <img src={resolvedSrc} alt={alt} loading="lazy" {...props} />;
      },
    }),
    [assets, headingIds],
  );
  return (
    <article className="article-markdown">
      <Markdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeHighlight]}
        components={markdownComponents}
      >
        {normalizedMarkdown}
      </Markdown>
    </article>
  );
}
