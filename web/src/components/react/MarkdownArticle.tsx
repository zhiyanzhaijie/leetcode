import { type ReactNode, useMemo } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';

type Props = {
  markdown: string;
  assets: Record<string, string>;
};

function normalizeMarkdown(markdown: string) {
  return markdown.replace(/\[img\]\(([^)]+)\)/g, '![img]($1)');
}

export default function MarkdownArticle({ markdown, assets }: Props) {
  const normalizedMarkdown = useMemo(() => normalizeMarkdown(markdown), [markdown]);
  const markdownComponents = useMemo(
    () => ({
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
    [assets],
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
