export interface FeatureFlagDetail {
  is_enabled: boolean;
  is_sub_feature: boolean;
  parent_id: string | null;
}

export type FeatureFlagsMap = Record<string, FeatureFlagDetail>;

let cachedFlags: FeatureFlagsMap | null = null;
let pendingFetch: Promise<FeatureFlagsMap> | null = null;

/**
 * Mengambil seluruh data feature flags langsung dari database backend (/api/v1/features).
 * Berisi status aktif (`is_enabled`), apakah sub-fitur (`is_sub_feature`), dan `parent_id`.
 */
export async function getFeatureFlags(forceRefresh = false): Promise<FeatureFlagsMap> {
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
      const serverData: FeatureFlagsMap = json.data || {};

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
 * Memeriksa apakah suatu fitur aktif (is_enabled === true).
 * Jika kunci tidak ada di database atau bernilai false, mengembalikan false.
 */
export async function isFeatureEnabled(key: string): Promise<boolean> {
  const flags = await getFeatureFlags();
  return flags[key]?.is_enabled === true;
}

/**
 * Memeriksa apakah suatu fitur merupakan sub-fitur (memiliki parent_id).
 */
export async function isSubFeature(key: string): Promise<boolean> {
  const flags = await getFeatureFlags();
  return flags[key]?.is_sub_feature === true;
}

/**
 * Mengambil detail lengkap suatu fitur (is_enabled, is_sub_feature, parent_id).
 */
export async function getFeatureDetail(key: string): Promise<FeatureFlagDetail | null> {
  const flags = await getFeatureFlags();
  return flags[key] ?? null;
}

/**
 * Menentukan aksi UI otomatis berdasarkan status database:
 * - 'active'               => Fitur aktif normal.
 * - 'redirect_maintenance' => Fitur induk mati => Alihkan halaman ke /maintenance.
 * - 'inline_alert'         => Sub-fitur mati => Tampilkan banner/alert di komponen tanpa redirect.
 */
export async function getFeatureAction(
  key: string
): Promise<'active' | 'redirect_maintenance' | 'inline_alert'> {
  const detail = await getFeatureDetail(key);
  if (!detail || !detail.is_enabled) {
    return detail?.is_sub_feature ? 'inline_alert' : 'redirect_maintenance';
  }
  return 'active';
}
