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
 * Mengambil daftar seluruh course yang dipublikasikan dari backend API.
 */
export async function fetchCourses(): Promise<Course[]> {
	try {
		const res = await fetch('/api/v1/courses');
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
