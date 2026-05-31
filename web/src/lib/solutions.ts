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

export const GITHUB_OWNER = 'zhiyanzhaijie';
export const GITHUB_REPO = 'leetcode';
export const GITHUB_BRANCH = 'main';
export const SOLUTIONS_DIR = 'solutions';

const RAW_BASE_URL = `https://raw.githubusercontent.com/${GITHUB_OWNER}/${GITHUB_REPO}/${GITHUB_BRANCH}`;
const TREE_API_URL = `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/git/trees/${GITHUB_BRANCH}?recursive=1`;
let solutionFilesPromise: Promise<SolutionFile[]> | null = null;
let solutionMetasPromise: Promise<SolutionMeta[]> | null = null;
const markdownCache = new Map<string, Promise<string>>();

type GitHubTreeResponse = {
  tree?: Array<{
    path?: string;
    type?: string;
  }>;
};

type SolutionFile = {
  id: number;
  path: string;
  markdownUrl: string;
  assetBaseUrl: string;
};

function extractTitle(body: string, id: number): string {
  const lines = body.split('\n').map((line) => line.trim()).filter(Boolean);
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

function extractTags(body: string): string[] {
  const headings = [...body.matchAll(/^##\s+(.+)$/gm)].map((match) => match[1].trim());
  const hints = ['stack', 'pointer', 'dp', 'graph', 'greedy', 'binary', 'tree', 'array', 'string'];
  const matched = hints.filter((hint) => new RegExp(`\\b${hint}\\b`, 'i').test(body));
  const fromHeadings = headings
    .map((heading) => heading.toLowerCase())
    .flatMap((heading) => {
      if (heading.includes('stack')) return ['stack'];
      if (heading.includes('pointer')) return ['two-pointers'];
      if (heading.includes('monotonic')) return ['monotonic'];
      if (heading.includes('double')) return ['double-pointer'];
      return [];
    });
  return [...new Set([...matched, ...fromHeadings])].slice(0, 3);
}

export function extractIdFromPath(path: string): number {
  const match = path.match(/^solutions\/(\d+)\/solution\.md$/);
  return match ? Number(match[1]) : Number.MAX_SAFE_INTEGER;
}

export function parseSolutionMeta(file: SolutionFile, markdown: string): SolutionMeta {
  return {
    id: file.id,
    slug: String(file.id),
    title: extractTitle(markdown, file.id),
    difficulty: extractDifficulty(markdown),
    summary: extractSummary(markdown),
    tags: extractTags(markdown),
    markdownUrl: file.markdownUrl,
    assetBaseUrl: file.assetBaseUrl,
  };
}

export function buildSolutionAssets(markdown: string, assetBaseUrl: string): Record<string, string> {
  const assets: Record<string, string> = {};
  const imageMatches = [
    ...markdown.matchAll(/!\[[^\]]*\]\((\.\/[^)]+)\)/g),
    ...markdown.matchAll(/\[img\]\((\.\/[^)]+)\)/g),
  ];

  for (const match of imageMatches) {
    const relativePath = match[1];
    assets[relativePath] = `${assetBaseUrl}/${relativePath.replace(/^\.\//, '')}`;
  }

  return assets;
}

export function sortByIdAsc<T extends { id: number }>(items: T[]): T[] {
  return [...items].sort((a, b) => a.id - b.id);
}

async function fetchText(url: string): Promise<string> {
  if (!markdownCache.has(url)) {
    markdownCache.set(url, (async () => {
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`Failed to fetch ${url}: ${response.status}`);
      }
      return response.text();
    })());
  }
  return markdownCache.get(url)!;
}

export async function fetchSolutionFiles(): Promise<SolutionFile[]> {
  if (solutionFilesPromise) return solutionFilesPromise;
  solutionFilesPromise = (async () => {
  const response = await fetch(TREE_API_URL, {
    headers: {
      Accept: 'application/vnd.github+json',
    },
  });

  if (!response.ok) {
    throw new Error(`Failed to list GitHub tree: ${response.status}`);
  }

  const data = (await response.json()) as GitHubTreeResponse;
  return (data.tree ?? [])
    .filter((entry) => entry.type === 'blob' && /^solutions\/\d+\/solution\.md$/.test(entry.path ?? ''))
    .map((entry) => {
      const path = entry.path ?? '';
      const id = extractIdFromPath(path);
      const solutionDir = path.replace(/\/solution\.md$/, '');
      return {
        id,
        path,
        markdownUrl: `${RAW_BASE_URL}/${path}`,
        assetBaseUrl: `${RAW_BASE_URL}/${solutionDir}`,
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
      const markdown = await fetchText(file.markdownUrl);
      return parseSolutionMeta(file, markdown);
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

  const markdown = await fetchText(file.markdownUrl);
  return {
    ...parseSolutionMeta(file, markdown),
    markdown,
    assets: buildSolutionAssets(markdown, file.assetBaseUrl),
  };
}
