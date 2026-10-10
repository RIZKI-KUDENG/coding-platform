// web/src/lib/admin-courses.ts
import { getAccessToken } from './auth';

export interface AdminCourse {
	id: string;
	title: string;
	slug: string;
	description: string | null;
	status: 'DRAFT' | 'PUBLISHED' | string;
	created_at: string;
	updated_at: string;
}

export interface CoursePayload {
	title: string;
	slug: string;
	description: string;
	status: string;
}

export interface EditCoursePayload {
	title: string;
	slug: string;
	description: string | null;
	status: string;
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
 * Mengambil semua course untuk Admin (termasuk status DRAFT dan PUBLISHED).
 */
export async function fetchAdminCourses(): Promise<{
	ok: boolean;
	data: AdminCourse[];
	status: number;
	error?: string;
}> {
	try {
		const res = await fetch('/api/v1/admin/courses', {
			method: 'GET',
			headers: getAuthHeader(),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				data: [],
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal memuat kursus`,
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
			error: err.message || 'Kesalahan jaringan saat memuat kursus admin.',
		};
	}
}

/**
 * Membuat course baru untuk Admin.
 */
export async function createAdminCourse(payload: CoursePayload): Promise<{
	ok: boolean;
	status: number;
	error?: string;
}> {
	try {
		const res = await fetch('/api/v1/admin/courses', {
			method: 'POST',
			headers: getAuthHeader(),
			body: JSON.stringify(payload),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal membuat kursus`,
			};
		}

		return {
			ok: true,
			status: res.status,
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Kesalahan jaringan saat membuat kursus.',
		};
	}
}

/**
 * Mengubah course untuk Admin.
 */
export async function editAdminCourse(
	id: string,
	payload: EditCoursePayload,
): Promise<{
	ok: boolean;
	status: number;
	error?: string;
}> {
	try {
		const res = await fetch(`/api/v1/admin/courses/${encodeURIComponent(id)}`, {
			method: 'PUT',
			headers: getAuthHeader(),
			body: JSON.stringify(payload),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal memperbarui kursus`,
			};
		}

		return {
			ok: true,
			status: res.status,
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Kesalahan jaringan saat memperbarui kursus.',
		};
	}
}

/**
 * Menghapus course untuk Admin.
 */
export async function deleteAdminCourse(id: string): Promise<{
	ok: boolean;
	status: number;
	error?: string;
}> {
	try {
		const res = await fetch(`/api/v1/admin/courses/${encodeURIComponent(id)}`, {
			method: 'DELETE',
			headers: getAuthHeader(),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json?.error?.message || `HTTP ${res.status}: Gagal menghapus kursus`,
			};
		}

		return {
			ok: true,
			status: res.status,
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Kesalahan jaringan saat menghapus kursus.',
		};
	}
}
