# i18n Additions for TASK-004

Please add the following keys to the respective translation files.

## src/lib/i18n/types.ts

```typescript
// Add to Translations interface
export interface Translations {
  // ... existing ...
  updateDialog: {
    title: string;
    newVersion: string;
    releaseNotes: string;
    updateNow: string;
    remindLater: string;
    skipVersion: string;
  };
}
```

## src/lib/i18n/locales/en.ts

```typescript
const en: Translations = {
  // ... existing ...
  updateDialog: {
    title: 'Update Available',
    newVersion: 'New Version',
    releaseNotes: 'What\'s New:',
    updateNow: 'Update Now',
    remindLater: 'Remind Me Later',
    skipVersion: 'Skip This Version',
  },
};
```

## src/lib/i18n/locales/zh-TW.ts

```typescript
const zhTW: Translations = {
  // ... existing ...
  updateDialog: {
    title: '有可用的更新',
    newVersion: '新版本',
    releaseNotes: '更新內容：',
    updateNow: '立即更新',
    remindLater: '稍後提醒',
    skipVersion: '略過此版本',
  },
};
```

