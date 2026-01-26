import type { VercelRequest, VercelResponse } from '@vercel/node';

interface FeedbackPayload {
  type: 'feature' | 'bug' | 'ux';
  title: string;
  description: string;
  version: string;
  platform: 'tauri' | 'web' | 'unknown';
  createdAt: string;
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

  // Type validation
  if (!['feature', 'bug', 'ux'].includes(type as string)) return null;
  if (typeof title !== 'string' || !title.trim()) return null;
  if (typeof description !== 'string' || !description.trim()) return null;
  if (typeof version !== 'string') return null;
  if (!['tauri', 'web', 'unknown'].includes(platform as string)) return null;
  if (typeof createdAt !== 'string') return null;

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

  // Log feedback (in production, would save to database)
  console.log('[Feedback]', JSON.stringify({
    ...payload,
    ip: clientIP,
    receivedAt: new Date().toISOString(),
  }));

  // TODO: In production, save to database or send to notification service
  // For now, just log and return success

  return res.status(200).json({ ok: true });
}
