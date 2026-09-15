// web/src/lib/auth.ts
export interface User {
	id: string;
	email: string;
	username: string;
}

const TOKEN_KEY = 'kodingan_access_token';
const USER_KEY = 'kodingan_user';

export function getAccessToken(): string | null {
	if (typeof window === 'undefined') return null;
	return localStorage.getItem(TOKEN_KEY);
}

export function getCurrentUser(): User | null {
	if (typeof window === 'undefined') return null;
	const userStr = localStorage.getItem(USER_KEY);
	if (!userStr) return null;
	try {
		return JSON.parse(userStr);
	} catch {
		return null;
	}
}

export function saveSession(user: User, token: string) {
	if (typeof window === 'undefined') return;
	localStorage.setItem(TOKEN_KEY, token);
	localStorage.setItem(USER_KEY, JSON.stringify(user));
	// Set cookie for potential server-side reading
	document.cookie = `kodingan_session=${token}; path=/; max-age=604800; SameSite=Lax`;
}

export function clearSession() {
	if (typeof window === 'undefined') return;
	localStorage.removeItem(TOKEN_KEY);
	localStorage.removeItem(USER_KEY);
	document.cookie = 'kodingan_session=; path=/; max-age=0; SameSite=Lax';
}

export function isAuthenticated(): boolean {
	return Boolean(getAccessToken());
}
