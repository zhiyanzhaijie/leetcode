export type MarkdownOutlineItem = {
  depth: 2 | 3;
  line: number;
  text: string;
  id: string;
};

function slugify(value: string): string {
  return value
    .toLowerCase()
    .replace(/[`*_~[\]()]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '') || 'section';
}

export function extractMarkdownOutline(markdown: string): MarkdownOutlineItem[] {
  const outline: MarkdownOutlineItem[] = [];
  const slugCounts = new Map<string, number>();
  let fence: string | null = null;

  markdown.split('\n').forEach((line, index) => {
    const fenceMatch = line.match(/^\s*(`{3,}|~{3,})/);
    if (fenceMatch) {
      const marker = fenceMatch[1][0];
      fence = fence === marker ? null : fence ?? marker;
      return;
    }
    if (fence) return;

    const heading = line.match(/^\s*(##|###)\s+(.+?)\s*#*\s*$/);
    if (!heading) return;

    const text = heading[2].trim();
    const baseSlug = slugify(text);
    const count = slugCounts.get(baseSlug) ?? 0;
    slugCounts.set(baseSlug, count + 1);

    outline.push({
      depth: heading[1].length as 2 | 3,
      line: index + 1,
      text,
      id: count === 0 ? baseSlug : `${baseSlug}-${count + 1}`,
    });
  });

  return outline;
}
