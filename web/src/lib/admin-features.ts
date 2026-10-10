// web/src/lib/admin-features.ts
import { getAccessToken } from './auth';

export interface AdminFeatureFlag {
	id: string;
	key: string;
	is_enabled: number; // 1 = aktif, 0 = nonaktif (maintenance)
	description: string | null;
	parent_id: string | null;
	created_at: string;
	updated_at: string;
}

export interface EditFeaturePayload {
	id: string;
	key: string;
	is_enabled: number;
	description: string;
}

function getAuthHeader(): Record<string, string> {
	const token = getAccessToken();
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
	};
	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}
	return headers;
}

/**
 * Mengambil daftar lengkap seluruh feature flags langsung dari backend admin.
 */
export async function fetchAdminFeatures(): Promise<{
	ok: boolean;
	data: AdminFeatureFlag[];
	status: number;
	error?: string;
}> {
	try {
		const res = await fetch('/api/v1/admin/features', {
			method: 'GET',
			headers: getAuthHeader(),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				data: [],
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal memuat status feature flag.`,
			};
		}

		return {
			ok: true,
			data: json.data || [],
			status: res.status,
		};
	} catch (err: any) {
		return {
			ok: false,
			data: [],
			status: 500,
			error: err.message || 'Kesalahan jaringan saat memuat feature flag.',
		};
	}
}

/**
 * Memperbarui status / deskripsi feature flag (Aktif / Nonaktifkan fitur).
 */
export async function updateAdminFeature(payload: EditFeaturePayload): Promise<{
	ok: boolean;
	status: number;
	message?: string;
	error?: string;
}> {
	try {
		const res = await fetch('/api/v1/admin/features', {
			method: 'PUT',
			headers: getAuthHeader(),
			body: JSON.stringify(payload),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal memperbarui feature flag.`,
			};
		}

		return {
			ok: true,
			status: res.status,
			message: json?.message || 'Fitur berhasil diperbarui.',
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Kesalahan jaringan saat memperbarui feature flag.',
		};
	}
}
