export type FeatureFlags = Record<string, boolean>;

let cachedFlags: FeatureFlags | null = null;
let pendingFetch: Promise<FeatureFlags> | null = null;

/**
 * Mengambil status feature flags murni langsung dari database backend (/api/v1/features).
 * Tanpa nilai default / fallback tiruan: apa yang ada di database, itulah yang dikembalikan.
 */
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
      if (!res.ok) {
        console.error('API /api/v1/features mengembalikan status non-OK:', res.status);
        return {};
      }
      const json = await res.json();
      const serverData: Record<string, boolean> = json.data || {};

      cachedFlags = { ...serverData };
      return cachedFlags;
    } catch (err) {
      console.error('Gagal mengambil data feature flags dari backend server:', err);
      return {};
    } finally {
      pendingFetch = null;
    }
  })();

  return pendingFetch;
}

/**
 * Memeriksa apakah suatu fitur bernilai true murni di database.
 * Jika kunci tidak terdaftar di database atau bernilai false (0), mengembalikan false.
 */
export async function isFeatureEnabled(key: string): Promise<boolean> {
  const flags = await getFeatureFlags();
  return flags[key] === true;
}
