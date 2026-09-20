// web/src/lib/courses.ts

export interface Course {
	id: string;
	title: string;
	slug: string;
	description: string | null;
	status: string;
	created_at: string;
	updated_at: string;
}

export interface Lesson {
	id: string;
	title: string;
	slug: string;
	description: string | null;
	status: string;
	created_at: string;
	updated_at: string;
	order?: number;
	completed?: boolean;
}

export interface CourseProgress {
	course_id: string;
	completed_lessons: number;
	total_lessons: number;
	percentage: number;
	recommended_lesson_id: string | null;
}

/**
 * Memeriksa apakah respons API mengindikasikan fitur dinonaktifkan (HTTP 503).
 * Jika ya, otomatis me-redirect browser ke halaman /maintenance?feature=courses.
 */
async function checkMaintenanceResponse(res: Response): Promise<boolean> {
	if (res.status === 503) {
		try {
			const json = await res.clone().json();
			const code = json?.error?.code;
			const feature = json?.error?.feature || 'courses';
			if (code === 'FEATURE_MAINTENANCE' || code === 'FEATURE_NOT_FOUND') {
				if (typeof window !== 'undefined') {
					window.location.replace(`/maintenance?feature=${encodeURIComponent(feature)}`);
					return true;
				}
			}
		} catch {
			if (typeof window !== 'undefined') {
				window.location.replace('/maintenance?feature=courses');
				return true;
			}
		}
	}
	return false;
}

/**
 * Mengambil daftar seluruh course yang dipublikasikan dari backend API.
 */
export async function fetchCourses(): Promise<Course[]> {
	try {
		const res = await fetch('/api/v1/courses');
		if (await checkMaintenanceResponse(res)) {
			return [];
		}
		if (!res.ok) {
			console.error(`Gagal mengambil data courses: HTTP ${res.status}`);
			return [];
		}
		const json = await res.json();
		return json.data || [];
	} catch (err) {
		console.error('Error saat fetchCourses:', err);
		return [];
	}
}

/**
 * Mengambil detail sebuah course berdasarkan slug.
 */
export async function fetchCourseBySlug(slug: string): Promise<Course | null> {
	try {
		const res = await fetch(`/api/v1/courses/slug/${encodeURIComponent(slug)}`);
		if (await checkMaintenanceResponse(res)) {
			return null;
		}
		if (!res.ok) {
			if (res.status === 404) return null;
			console.error(`Gagal mengambil course slug ${slug}: HTTP ${res.status}`);
			return null;
		}
		const json = await res.json();
		return json.data || null;
	} catch (err) {
		console.error(`Error saat fetchCourseBySlug (${slug}):`, err);
		return null;
	}
}

/**
 * Mengambil detail sebuah course berdasarkan ID UUID.
 */
export async function fetchCourseById(id: string): Promise<Course | null> {
	try {
		const res = await fetch(`/api/v1/courses/${encodeURIComponent(id)}`);
		if (await checkMaintenanceResponse(res)) {
			return null;
		}
		if (!res.ok) {
			if (res.status === 404) return null;
			console.error(`Gagal mengambil course id ${id}: HTTP ${res.status}`);
			return null;
		}
		const json = await res.json();
		return json.data || null;
	} catch (err) {
		console.error(`Error saat fetchCourseById (${id}):`, err);
		return null;
	}
}

/**
 * Mengambil daftar lessons untuk course tertentu berdasarkan course_id.
 */
export async function fetchCourseLessons(courseId: string): Promise<Lesson[]> {
	try {
		const res = await fetch(`/api/v1/courses/${encodeURIComponent(courseId)}/lessons`);
		if (await checkMaintenanceResponse(res)) {
			return [];
		}
		if (!res.ok) {
			console.error(`Gagal mengambil lessons untuk course ${courseId}: HTTP ${res.status}`);
			return [];
		}
		const json = await res.json();
		return json.data || [];
	} catch (err) {
		console.error(`Error saat fetchCourseLessons (${courseId}):`, err);
		return [];
	}
}

/**
 * Mengambil detail lesson berdasarkan slug.
 */
export async function fetchLessonBySlug(slug: string): Promise<Lesson | null> {
	try {
		const res = await fetch(`/api/v1/lessons/slug/${encodeURIComponent(slug)}`);
		if (await checkMaintenanceResponse(res)) {
			return null;
		}
		if (!res.ok) {
			if (res.status === 404) return null;
			console.error(`Gagal mengambil lesson slug ${slug}: HTTP ${res.status}`);
			return null;
		}
		const json = await res.json();
		return json.data || null;
	} catch (err) {
		console.error(`Error saat fetchLessonBySlug (${slug}):`, err);
		return null;
	}
}

/**
 * Mengambil detail lesson berdasarkan ID UUID.
 */
export async function fetchLessonById(id: string): Promise<Lesson | null> {
	try {
		const res = await fetch(`/api/v1/lessons/${encodeURIComponent(id)}`);
		if (await checkMaintenanceResponse(res)) {
			return null;
		}
		if (!res.ok) {
			if (res.status === 404) return null;
			console.error(`Gagal mengambil lesson id ${id}: HTTP ${res.status}`);
			return null;
		}
		const json = await res.json();
		return json.data || null;
	} catch (err) {
		console.error(`Error saat fetchLessonById (${id}):`, err);
		return null;
	}
}
