export type Difficulty = 'Easy' | 'Medium' | 'Hard' | 'Unknown';

export type SolutionMeta = {
  id: number;
  slug: string;
  title: string;
  difficulty: Difficulty;
  summary: string;
  tags: string[];
  markdownUrl: string;
  assetBaseUrl: string;
};

export type Solution = SolutionMeta & {
  markdown: string;
  assets: Record<string, string>;
};
export const SOLUTIONS_DIR = 'solutions';
let solutionFilesPromise: Promise<SolutionFile[]> | null = null;
let solutionMetasPromise: Promise<SolutionMeta[]> | null = null;
const tagGroupsMarkdownModules = import.meta.glob('../solutions/tag_groups.md', {
  query: '?raw',
  import: 'default',
  eager: true,
}) as Record<string, string>;
const solutionMarkdownModules = import.meta.glob('../solutions/*/solution.md', {
  query: '?raw',
  import: 'default',
  eager: true,
}) as Record<string, string>;
const solutionAssetModules = import.meta.glob('../solutions/**/*', {
  query: '?url',
  import: 'default',
  eager: true,
}) as Record<string, string>;

type SolutionFile = {
  id: number;
  path: string;
  markdownUrl: string;
  assetBaseUrl: string;
  markdown: string;
};

function extractTitle(body: string, id: number): string {
  const lines = body.split('\n').map((line) => line.trim()).filter(Boolean);
  const heading = lines.find((line) => new RegExp(`^#\\s+${id}\\.\\s+`).test(line));
  if (heading) {
    return heading.replace(new RegExp(`^#\\s+${id}\\.\\s+`), '').trim();
  }

  const numbered = lines.find((line) => new RegExp(`^${id}\\.\\s+`).test(line));
  if (numbered) {
    return numbered.replace(new RegExp(`^${id}\\.\\s+`), '').trim();
  }

  const h2 = lines.find((line) => line.startsWith('## '));
  if (h2) {
    return h2.replace(/^##\s+/, '').trim();
  }

  return `Problem ${id}`;
}

function extractDifficulty(body: string): Difficulty {
  if (/\bEasy\b/i.test(body)) return 'Easy';
  if (/\bMedium\b/i.test(body)) return 'Medium';
  if (/\bHard\b/i.test(body)) return 'Hard';
  return 'Unknown';
}

function extractSummary(body: string): string {
  const normalized = body
    .replace(/```[\s\S]*?```/g, '')
    .replace(/!\[[^\]]*\]\([^)]+\)/g, '')
    .replace(/\[img\]\([^)]+\)/g, '')
    .replace(/\s+/g, ' ')
    .trim();

  if (!normalized) return 'Notes for thinking and re-thinking.';

  const parts = normalized
    .split(/(?<=[.!?。！？])\s+/)
    .map((part) => part.trim())
    .filter(Boolean);

  if (parts.length === 0) return 'Notes for thinking and re-thinking.';
  return parts[0].slice(0, 120);
}

function parseTagGroups(body: string): Map<number, string[]> {
  const tagsByProblem = new Map<number, string[]>();
  let currentTag: string | null = null;

  for (const line of body.split('\n')) {
    const subgroup = line.match(/^###\s+\d+\.\d+\s+(.+?)\s*$/);
    if (subgroup) {
      currentTag = subgroup[1].trim();
      continue;
    }

    if (/^#{1,2}\s+/.test(line) || /^---\s*$/.test(line)) {
      currentTag = null;
      continue;
    }

    if (!currentTag) continue;
    const problem = line.match(
      /^-\s+(?:\[[ xX]\]\s+)?(?:\[(?:Easy|Medium|Hard)\]\s+|(?:Easy|Medium|Hard)\s+)?(\d+)\./,
    );
    if (!problem) continue;

    const problemId = Number(problem[1]);
    const tags = tagsByProblem.get(problemId) ?? [];
    if (!tags.includes(currentTag)) tags.push(currentTag);
    tagsByProblem.set(problemId, tags);
  }

  return tagsByProblem;
}

const tagGroupsMarkdown = Object.values(tagGroupsMarkdownModules)[0] ?? '';
const tagsByProblem = parseTagGroups(tagGroupsMarkdown);

export function extractIdFromPath(path: string): number {
  const match = path.match(/(?:^|\/)solutions\/(\d+)\/solution\.md$/);
  return match ? Number(match[1]) : Number.MAX_SAFE_INTEGER;
}

export function parseSolutionMeta(file: SolutionFile, markdown: string): SolutionMeta {
  return {
    id: file.id,
    slug: String(file.id),
    title: extractTitle(markdown, file.id),
    difficulty: extractDifficulty(markdown),
    summary: extractSummary(markdown),
    tags: tagsByProblem.get(file.id) ?? [],
    markdownUrl: file.markdownUrl,
    assetBaseUrl: file.assetBaseUrl,
  };
}

export function buildSolutionAssets(markdown: string, assetBaseUrl: string): Record<string, string> {
  const assets: Record<string, string> = {};
  const solutionDir = assetBaseUrl.replace(/^\/+/, '');
  const imageMatches = [
    ...markdown.matchAll(/!\[[^\]]*\]\((\.\/[^)]+)\)/g),
    ...markdown.matchAll(/\[img\]\((\.\/[^)]+)\)/g),
  ];

  for (const match of imageMatches) {
    const relativePath = match[1];
    const modulePath = `../${solutionDir}/${relativePath.replace(/^\.\//, '')}`;
    const resolvedUrl = solutionAssetModules[modulePath];
    assets[relativePath] = resolvedUrl ?? `${assetBaseUrl}/${relativePath.replace(/^\.\//, '')}`;
  }

  return assets;
}

export function sortByIdAsc<T extends { id: number }>(items: T[]): T[] {
  return [...items].sort((a, b) => a.id - b.id);
}


export async function fetchSolutionFiles(): Promise<SolutionFile[]> {
  if (solutionFilesPromise) return solutionFilesPromise;
  solutionFilesPromise = (async () => {
  return Object.entries(solutionMarkdownModules)
    .map(([modulePath, markdown]) => {
      const id = extractIdFromPath(modulePath);
      const path = `${SOLUTIONS_DIR}/${id}/solution.md`;
      const solutionDir = `${SOLUTIONS_DIR}/${id}`;
      return {
        id,
        path,
        markdownUrl: `/${path}`,
        assetBaseUrl: `/${solutionDir}`,
        markdown,
      };
    })
    .filter((file) => Number.isFinite(file.id))
    .sort((a, b) => a.id - b.id);
  })();
  return solutionFilesPromise;
}

export async function fetchSolutionMetas(): Promise<SolutionMeta[]> {
  if (solutionMetasPromise) return solutionMetasPromise;
  solutionMetasPromise = (async () => {
  const files = await fetchSolutionFiles();
  const metas = await Promise.all(
    files.map(async (file) => {
      return parseSolutionMeta(file, file.markdown);
    }),
  );
  return sortByIdAsc(metas);
  })();
  return solutionMetasPromise;
}

export async function fetchSolutionById(id: number): Promise<Solution | null> {
  const files = await fetchSolutionFiles();
  const file = files.find((item) => item.id === id);
  if (!file) return null;
  const markdown = file.markdown;
  return {
    ...parseSolutionMeta(file, markdown),
    markdown,
    assets: buildSolutionAssets(markdown, file.assetBaseUrl),
  };
}
