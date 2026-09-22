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

export interface Exercise {
	id: string;
	lesson_id: string;
	slug: string;
	title: string;
	description: string | null;
	starter_code: string | null;
	language: string;
	order: number;
	xp_reward: number;
	status: string;
	created_at: string;
	updated_at: string;
}

export interface SubmissionResult {
	success: boolean;
	stdout: string;
	stderr: string;
	exit_code: number | null;
	duration_ms: number;
}

/**
 * Mengambil daftar exercises untuk sebuah lesson.
 */
export async function fetchLessonExercises(lessonId: string): Promise<Exercise[]> {
	try {
		const res = await fetch(`/api/v1/lessons/${encodeURIComponent(lessonId)}/exercise`);
		if (await checkMaintenanceResponse(res)) {
			return [];
		}
		if (!res.ok) {
			if (res.status === 404) return [];
			console.error(`Gagal mengambil exercises untuk lesson ${lessonId}: HTTP ${res.status}`);
			return [];
		}
		const json = await res.json();
		return json.data || [];
	} catch (err) {
		console.error(`Error saat fetchLessonExercises (${lessonId}):`, err);
		return [];
	}
}

/**
 * Mengambil detail exercise berdasarkan ID.
 */
export async function fetchExerciseById(exerciseId: string): Promise<Exercise | null> {
	try {
		const res = await fetch(`/api/v1/exercise/${encodeURIComponent(exerciseId)}`);
		if (await checkMaintenanceResponse(res)) {
			return null;
		}
		if (!res.ok) {
			if (res.status === 404) return null;
			console.error(`Gagal mengambil exercise ${exerciseId}: HTTP ${res.status}`);
			return null;
		}
		const json = await res.json();
		return json.data || null;
	} catch (err) {
		console.error(`Error saat fetchExerciseById (${exerciseId}):`, err);
		return null;
	}
}

/**
 * Mengirimkan submission kode untuk exercise (uji terhadap seluruh test cases).
 */
export async function submitExerciseCode(
	exerciseId: string,
	code: string,
	language: string,
	token?: string | null,
): Promise<{ ok: boolean; status: number; data?: SubmissionResult; error?: string }> {
	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
		};
		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const res = await fetch(`/api/v1/exercises/${encodeURIComponent(exerciseId)}/submissions`, {
			method: 'POST',
			headers,
			body: JSON.stringify({ code, language }),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json.error?.message || `HTTP ${res.status}: Gagal mengirim solusi.`,
			};
		}

		return {
			ok: true,
			status: res.status,
			data: json.data,
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Terjadi kesalahan jaringan saat mengirim solusi.',
		};
	}
}

/**
 * Menjalankan kode secara bebas tanpa evaluasi test case (Run/Playground runner).
 */
export async function runPlaygroundCode(
	code: string,
	language: string,
): Promise<{ ok: boolean; status: number; data?: SubmissionResult; error?: string }> {
	try {
		const res = await fetch('/api/v1/playground/run', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({ code, language }),
		});

		const json = await res.json().catch(() => ({}));
		if (!res.ok) {
			return {
				ok: false,
				status: res.status,
				error: json.error?.message || `HTTP ${res.status}: Gagal mengeksekusi kode.`,
			};
		}

		return {
			ok: true,
			status: res.status,
			data: json.data,
		};
	} catch (err: any) {
		return {
			ok: false,
			status: 500,
			error: err.message || 'Terjadi kesalahan jaringan saat mengeksekusi kode.',
		};
	}
}

