export interface FeatureFlags {
  playground: boolean;
  courses: boolean;
  practice: boolean;
  gamification: boolean;
  [key: string]: boolean | undefined;
}

const DEFAULT_FLAGS: FeatureFlags = {
  playground: true,
  courses: false,
  practice: false,
  gamification: false,
};

let cachedFlags: FeatureFlags | null = null;
let pendingFetch: Promise<FeatureFlags> | null = null;

export async function getFeatureFlags(forceRefresh = false): Promise<FeatureFlags> {
  if (cachedFlags && !forceRefresh) {
    return cachedFlags;
  }

  if (pendingFetch && !forceRefresh) {
    return pendingFetch;
  }

  pendingFetch = (async () => {
    try {
      const res = await fetch('/api/v1/features');
      if (!res.ok) return DEFAULT_FLAGS;
      const json = await res.json();
      const serverData = json.data || {};

      cachedFlags = {
        ...DEFAULT_FLAGS,
        ...serverData,
        // Dukung alias 'course' dan 'courses'
        course: serverData.courses ?? serverData.course ?? DEFAULT_FLAGS.courses,
        courses: serverData.courses ?? serverData.course ?? DEFAULT_FLAGS.courses,
      };
      return cachedFlags;
    } catch (err) {
      console.warn('Gagal memuat status feature flags, menggunakan default:', err);
      return DEFAULT_FLAGS;
    } finally {
      pendingFetch = null;
    }
  })();

  return pendingFetch;
}

export async function isFeatureEnabled(key: keyof FeatureFlags | string, defaultVal = false): Promise<boolean> {
  const flags = await getFeatureFlags();
  const val = flags[key];
  return typeof val === 'boolean' ? val : defaultVal;
}
