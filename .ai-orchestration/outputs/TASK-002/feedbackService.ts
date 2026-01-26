export type FeedbackType = 'feature' | 'bug' | 'ux';

export interface FeedbackPayload {
  type: FeedbackType;
  title: string;
  description: string;
  version: string;
  platform: 'tauri' | 'web' | 'unknown';
  createdAt: string;
}

interface QueueItem extends FeedbackPayload {
  id: string;
  attempts: number;
  nextRetryAt: number;
}

export interface SubmitResult {
  ok: boolean;
  queued: boolean;
  error?: string;
}

export interface SubmitOptions {
  endpoint?: string;
  forceFlush?: boolean;
}

const DB_NAME = 'bankflow_feedback';
const STORE_NAME = 'queue';
const DB_VERSION = 1;
const STORAGE_KEY = 'bankflow_feedback_queue';
const DEFAULT_ENDPOINT = '/api/feedback';
const REQUEST_TIMEOUT_MS = 5000;
const MAX_RETRIES = 5;
const RETRY_BASE_MS = 1000;
const RETRY_MAX_MS = 30000;

function canUseIndexedDB(): boolean {
  return typeof indexedDB !== 'undefined';
}

function createId(): string {
  if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
    return crypto.randomUUID();
  }
  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function openDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: 'id' });
      }
    };

    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

function requestToPromise<T>(request: IDBRequest<T>): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

function transactionDone(tx: IDBTransaction): Promise<void> {
  return new Promise((resolve, reject) => {
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
    tx.onabort = () => reject(tx.error);
  });
}

async function idbGetAll(): Promise<QueueItem[]> {
  const db = await openDb();
  const tx = db.transaction(STORE_NAME, 'readonly');
  const store = tx.objectStore(STORE_NAME);
  const request = store.getAll() as IDBRequest<QueueItem[]>;
  const result = await requestToPromise(request);
  await transactionDone(tx);
  return result ?? [];
}

async function idbPut(item: QueueItem): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(STORE_NAME, 'readwrite');
  const store = tx.objectStore(STORE_NAME);
  store.put(item);
  await transactionDone(tx);
}

async function idbDelete(id: string): Promise<void> {
  const db = await openDb();
  const tx = db.transaction(STORE_NAME, 'readwrite');
  const store = tx.objectStore(STORE_NAME);
  store.delete(id);
  await transactionDone(tx);
}

function getLocalQueue(): QueueItem[] {
  if (typeof localStorage === 'undefined') return [];
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) return [];
  try {
    return JSON.parse(raw) as QueueItem[];
  } catch {
    return [];
  }
}

function setLocalQueue(items: QueueItem[]): void {
  if (typeof localStorage === 'undefined') return;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(items));
}

async function saveQueueItem(item: QueueItem): Promise<void> {
  if (canUseIndexedDB()) {
    await idbPut(item);
    return;
  }
  const queue = getLocalQueue();
  queue.push(item);
  setLocalQueue(queue);
}

async function listQueueItems(): Promise<QueueItem[]> {
  if (canUseIndexedDB()) {
    return idbGetAll();
  }
  return getLocalQueue();
}

async function removeQueueItem(id: string): Promise<void> {
  if (canUseIndexedDB()) {
    await idbDelete(id);
    return;
  }
  const queue = getLocalQueue().filter((item) => item.id !== id);
  setLocalQueue(queue);
}

async function updateQueueItem(item: QueueItem): Promise<void> {
  await saveQueueItem(item);
}

function validatePayload(payload: FeedbackPayload): string | null {
  if (!payload.type) return 'type';
  if (!payload.title.trim()) return 'title';
  if (!payload.description.trim()) return 'description';
  return null;
}

function computeNextRetry(attempts: number): number {
  const delay = Math.min(RETRY_MAX_MS, RETRY_BASE_MS * Math.pow(2, attempts));
  return Date.now() + delay;
}

async function postFeedback(payload: FeedbackPayload, endpoint: string): Promise<void> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);

  try {
    const res = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
      signal: controller.signal,
    });

    if (!res.ok) {
      throw new Error(`Feedback submit failed: ${res.status}`);
    }
  } finally {
    clearTimeout(timeout);
  }
}

export async function enqueueFeedback(payload: FeedbackPayload): Promise<void> {
  const item: QueueItem = {
    ...payload,
    id: createId(),
    attempts: 0,
    nextRetryAt: Date.now(),
  };

  await saveQueueItem(item);
}

export async function flushQueue(endpoint: string = DEFAULT_ENDPOINT): Promise<void> {
  const queue = await listQueueItems();
  for (const item of queue) {
    if (Date.now() < item.nextRetryAt) {
      continue;
    }

    try {
      await postFeedback(item, endpoint);
      await removeQueueItem(item.id);
    } catch (error) {
      const attempts = item.attempts + 1;
      const nextRetryAt = computeNextRetry(attempts);
      await updateQueueItem({
        ...item,
        attempts,
        nextRetryAt,
      });
    }
  }
}

export async function submitFeedback(
  payload: FeedbackPayload,
  options: SubmitOptions = {}
): Promise<SubmitResult> {
  const validationError = validatePayload(payload);
  if (validationError) {
    return { ok: false, queued: false, error: validationError };
  }

  await enqueueFeedback(payload);

  const endpoint = options.endpoint ?? DEFAULT_ENDPOINT;
  const online = typeof navigator !== 'undefined' ? navigator.onLine : true;

  if (!online && !options.forceFlush) {
    return { ok: true, queued: true };
  }

  try {
    await flushQueue(endpoint);
    return { ok: true, queued: false };
  } catch (error) {
    return { ok: false, queued: true, error: 'network' };
  }
}
