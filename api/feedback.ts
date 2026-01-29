import type { VercelRequest, VercelResponse } from '@vercel/node';

interface FeedbackPayload {
  type: 'feature' | 'bug' | 'ux';
  title: string;
  description: string;
  version: string;
  platform: 'tauri' | 'web' | 'unknown';
  createdAt: string;
  attachments?: string[];
}

interface StorageResult {
  ok: boolean;
  issueCreated: boolean;
  dbStored: boolean;
}

// Rate limiting: simple in-memory store (resets on cold start)
const rateLimitStore = new Map<string, { count: number; resetAt: number }>();
const RATE_LIMIT = 10; // requests per window
const RATE_WINDOW_MS = 60 * 1000; // 1 minute

function getClientIP(req: VercelRequest): string {
  const forwarded = req.headers['x-forwarded-for'];
  if (typeof forwarded === 'string') {
    return forwarded.split(',')[0].trim();
  }
  return req.socket?.remoteAddress ?? 'unknown';
}

function isRateLimited(ip: string): boolean {
  const now = Date.now();
  const entry = rateLimitStore.get(ip);

  if (!entry || now > entry.resetAt) {
    rateLimitStore.set(ip, { count: 1, resetAt: now + RATE_WINDOW_MS });
    return false;
  }

  if (entry.count >= RATE_LIMIT) {
    return true;
  }

  entry.count++;
  return false;
}

function validatePayload(body: unknown): FeedbackPayload | null {
  if (!body || typeof body !== 'object') return null;

  const payload = body as Record<string, unknown>;

  // Required fields
  const type = payload.type;
  const title = payload.title;
  const description = payload.description;
  const version = payload.version;
  const platform = payload.platform;
  const createdAt = payload.createdAt;
  const attachments = payload.attachments;

  // Type validation
  if (!['feature', 'bug', 'ux'].includes(type as string)) return null;
  if (typeof title !== 'string' || !title.trim()) return null;
  if (typeof description !== 'string' || !description.trim()) return null;
  if (typeof version !== 'string') return null;
  if (!['tauri', 'web', 'unknown'].includes(platform as string)) return null;
  if (typeof createdAt !== 'string') return null;
  if (attachments !== undefined) {
    if (!Array.isArray(attachments)) return null;
    if (attachments.length > 20) return null;
    const invalid = attachments.find((item) => {
      if (typeof item !== 'string' || !item.trim()) return true;
      try {
        const url = new URL(item);
        return url.protocol !== 'http:' && url.protocol !== 'https:';
      } catch {
        return true;
      }
    });
    if (invalid) return null;
  }

  // Length limits
  if (title.length > 200) return null;
  if (description.length > 5000) return null;

  return {
    type: type as FeedbackPayload['type'],
    title: title.trim(),
    description: description.trim(),
    version: version.slice(0, 20),
    platform: platform as FeedbackPayload['platform'],
    createdAt,
    attachments: attachments ? attachments.map((item) => item.trim()) : undefined,
  };
}

function buildIssueTitle(payload: FeedbackPayload): string {
  return `[Feedback/${payload.type}] ${payload.title}`;
}

function buildIssueBody(payload: FeedbackPayload): string {
  const attachments = payload.attachments?.length
    ? `## Attachments\n${payload.attachments.map((item) => `- ${item}`).join('\n')}\n\n---\n\n`
    : '';

  return [
    '## Feedback',
    `- Type: ${payload.type}`,
    `- Version: ${payload.version}`,
    `- Platform: ${payload.platform}`,
    `- CreatedAt: ${payload.createdAt}`,
    '',
    '---',
    '',
    attachments + payload.description,
  ].join('\n');
}

async function createGitHubIssue(payload: FeedbackPayload): Promise<boolean> {
  const token = process.env.FEEDBACK_GITHUB_TOKEN;
  if (!token) return false;

  const repo = process.env.FEEDBACK_GITHUB_REPO ?? 'Birdman1972/BankFlow-Tactical-Analyzer';
  const [owner, name] = repo.split('/');
  if (!owner || !name) return false;

  const response = await fetch(`https://api.github.com/repos/${owner}/${name}/issues`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
      Accept: 'application/vnd.github+json',
      'Content-Type': 'application/json',
      'X-GitHub-Api-Version': '2022-11-28',
    },
    body: JSON.stringify({
      title: buildIssueTitle(payload),
      body: buildIssueBody(payload),
      labels: ['feedback', `type:${payload.type}`, `platform:${payload.platform}`],
    }),
  });

  return response.ok;
}

async function storeInDb(payload: FeedbackPayload): Promise<boolean> {
  const endpoint = process.env.FEEDBACK_DB_ENDPOINT;
  if (!endpoint) return false;

  const token = process.env.FEEDBACK_DB_TOKEN;
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify({
      ...payload,
      receivedAt: new Date().toISOString(),
    }),
  });

  return response.ok;
}

async function storeFeedback(payload: FeedbackPayload): Promise<StorageResult> {
  let issueCreated = false;
  let dbStored = false;

  try {
    issueCreated = await createGitHubIssue(payload);
  } catch (error) {
    console.warn('[Feedback] GitHub issue failed:', error);
  }

  try {
    dbStored = await storeInDb(payload);
  } catch (error) {
    console.warn('[Feedback] DB storage failed:', error);
  }

  return {
    ok: issueCreated || dbStored,
    issueCreated,
    dbStored,
  };
}

export default async function handler(
  req: VercelRequest,
  res: VercelResponse
) {
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  // Handle preflight
  if (req.method === 'OPTIONS') {
    return res.status(200).end();
  }

  // Only allow POST
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  // Rate limiting
  const clientIP = getClientIP(req);
  if (isRateLimited(clientIP)) {
    return res.status(429).json({ error: 'Too many requests' });
  }

  // Validate payload
  const payload = validatePayload(req.body);
  if (!payload) {
    return res.status(400).json({ error: 'Invalid payload' });
  }

  // Log feedback (avoid sensitive data)
  console.log('[Feedback]', JSON.stringify({
    ...payload,
    ip: clientIP,
    receivedAt: new Date().toISOString(),
  }));

  const result = await storeFeedback(payload);
  if (!result.ok) {
    return res.status(502).json({ ok: false, error: 'storage_failed' });
  }

  return res.status(200).json({
    ok: true,
    issueCreated: result.issueCreated,
    dbStored: result.dbStored,
  });
}
