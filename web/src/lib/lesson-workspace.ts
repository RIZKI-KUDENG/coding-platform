// web/src/lib/lesson-workspace.ts
import {
	fetchLessonBySlug,
	fetchLessonById,
	fetchLessonExercises,
	submitExerciseCode,
	runPlaygroundCode,
	type Lesson,
	type Exercise,
} from '@/lib/courses';
import { getAccessToken, isAuthenticated } from '@/lib/auth';
import { isFeatureEnabled } from '@/lib/feature-flags';
import { EditorView, basicSetup } from 'codemirror';
import { EditorState, Compartment } from '@codemirror/state';
import { python } from '@codemirror/lang-python';
import { javascript } from '@codemirror/lang-javascript';
import { rust } from '@codemirror/lang-rust';
import { oneDark } from '@codemirror/theme-one-dark';

function getLanguageExtension(lang: string) {
	const l = (lang || '').toLowerCase();
	switch (l) {
		case 'javascript':
		case 'js':
		case 'node':
			return javascript();
		case 'rust':
			return rust();
		case 'python':
		case 'py':
		default:
			return python();
	}
}

function getFilenameForLang(lang: string): string {
	const l = (lang || '').toLowerCase();
	if (l.includes('javascript') || l.includes('js')) return 'solution.js';
	if (l.includes('rust')) return 'main.rs';
	return 'solution.py';
}

function escapeHtml(str: string): string {
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&#039;');
}

let currentExercises: Exercise[] = [];
let activeExercise: Exercise | null = null;
let editorView: EditorView | null = null;
const languageCompartment = new Compartment();

export async function initLessonWorkspace() {
	try {
		const active = await isFeatureEnabled('courses');
		if (!active) {
			window.location.replace('/maintenance?feature=courses');
			return;
		}
	} catch (err) {
		console.warn('Feature flag check error:', err);
	}

	const params = new URLSearchParams(window.location.search);
	const courseSlug = params.get('course');
	const lessonSlug = params.get('lesson');
	const lessonId = params.get('id');

	const loadingEl = document.getElementById('lesson-loading');
	const errorEl = document.getElementById('lesson-error');
	const errorMsgEl = document.getElementById('lesson-error-msg');
	const contentEl = document.getElementById('lesson-content');
	const backBtn = document.getElementById('btn-back-course') as HTMLAnchorElement | null;
	const bottomBackBtn = document.getElementById('bottom-back-link') as HTMLAnchorElement | null;

	if (courseSlug) {
		const backUrl = `/courses/detail?slug=${encodeURIComponent(courseSlug)}`;
		if (backBtn) backBtn.href = backUrl;
		if (bottomBackBtn) bottomBackBtn.href = backUrl;
	}

	if (!lessonSlug && !lessonId) {
		if (loadingEl) loadingEl.classList.add('hidden');
		if (errorEl) {
			errorEl.classList.remove('hidden');
			if (errorMsgEl) errorMsgEl.textContent = 'Parameter materi pelajaran tidak disertakan pada URL.';
		}
		return;
	}

	try {
		let lesson: Lesson | null = null;
		if (lessonSlug) {
			lesson = await fetchLessonBySlug(lessonSlug);
		} else if (lessonId) {
			lesson = await fetchLessonById(lessonId);
		}

		if (!lesson) {
			if (loadingEl) loadingEl.classList.add('hidden');
			if (errorEl) errorEl.classList.remove('hidden');
			return;
		}

		// Render Lesson Header
		const titleEl = document.getElementById('lesson-title');
		const descEl = document.getElementById('lesson-desc');
		const badgeEl = document.getElementById('lesson-header-badge');
		const materialText = document.getElementById('lesson-material-text');

		if (titleEl) titleEl.textContent = lesson.title;
		if (descEl) descEl.textContent = lesson.description || 'Tidak ada ringkasan materi tambahan untuk pelajaran ini.';
		if (badgeEl) badgeEl.textContent = `LESSON // ${lesson.slug}`;
		if (materialText && lesson.description) {
			materialText.textContent = lesson.description;
		}

		// Fetch Exercises for this Lesson
		currentExercises = await fetchLessonExercises(lesson.id);
		const counterBadge = document.getElementById('exercise-counter-badge');
		if (counterBadge) {
			counterBadge.textContent = `${currentExercises.length} LATIHAN TERSEDIA`;
		}

		setupExerciseTabs();
		setupCodeEditor();

		if (loadingEl) loadingEl.classList.add('hidden');
		if (contentEl) contentEl.classList.remove('hidden');

		setupExecutionActions();
	} catch (err: any) {
		if (loadingEl) loadingEl.classList.add('hidden');
		if (errorEl) {
			errorEl.classList.remove('hidden');
			if (errorMsgEl) errorMsgEl.textContent = err.message || 'Gagal memuat materi pelajaran.';
		}
	}
}

function setupExerciseTabs() {
	const tabsContainer = document.getElementById('exercise-tabs-container');
	const tabsList = document.getElementById('exercise-tabs-list');
	if (!tabsList || !tabsContainer) return;

	if (currentExercises.length === 0) {
		tabsContainer.classList.add('hidden');
		renderFallbackExercise();
		return;
	}

	tabsContainer.classList.remove('hidden');
	tabsList.innerHTML = '';

	currentExercises.forEach((ex, idx) => {
		const btn = document.createElement('button');
		btn.type = 'button';
		btn.className = `px-3 py-1.5 font-mono text-xs font-bold border transition-all cursor-pointer flex items-center gap-1.5 ${
			idx === 0
				? 'bg-[#facc15] text-[#3c2f00] border-black shadow-[2px_2px_0px_#000000]'
				: 'bg-[#0e0e11] text-[#d1c6ab] border-[#353438] hover:text-[#e4e1e6] hover:border-[#facc15]'
		}`;
		btn.innerHTML = `<span>#${idx + 1}</span> <span>${escapeHtml(ex.title)}</span>`;

		btn.addEventListener('click', () => {
			selectExercise(ex, btn);
		});

		tabsList.appendChild(btn);
	});

	selectExercise(currentExercises[0], tabsList.children[0] as HTMLButtonElement);
}

function selectExercise(ex: Exercise, activeTabBtn?: HTMLButtonElement) {
	activeExercise = ex;

	const tabsList = document.getElementById('exercise-tabs-list');
	if (tabsList && activeTabBtn) {
		Array.from(tabsList.children).forEach((child) => {
			const b = child as HTMLButtonElement;
			b.className =
				'px-3 py-1.5 font-mono text-xs font-bold border transition-all cursor-pointer flex items-center gap-1.5 bg-[#0e0e11] text-[#d1c6ab] border-[#353438] hover:text-[#e4e1e6] hover:border-[#facc15]';
		});
		activeTabBtn.className =
			'px-3 py-1.5 font-mono text-xs font-bold border transition-all cursor-pointer flex items-center gap-1.5 bg-[#facc15] text-[#3c2f00] border-black shadow-[2px_2px_0px_#000000]';
	}

	const titleEl = document.getElementById('exercise-title');
	const descEl = document.getElementById('exercise-desc');
	const xpBadge = document.getElementById('exercise-xp-badge');
	const langBadge = document.getElementById('exercise-lang-badge');
	const tabFilename = document.getElementById('editor-tab-filename');

	if (titleEl) titleEl.textContent = ex.title;
	if (descEl) descEl.textContent = ex.description || 'Tidak ada instruksi detail untuk latihan ini.';
	if (xpBadge) xpBadge.textContent = `+${ex.xp_reward || 100} XP`;
	if (langBadge) langBadge.textContent = (ex.language || 'PYTHON').toUpperCase();
	if (tabFilename) tabFilename.textContent = getFilenameForLang(ex.language);

	if (editorView) {
		const starter = ex.starter_code || `# Tulis kode solusi Anda di sini\n`;
		editorView.dispatch({
			changes: {
				from: 0,
				to: editorView.state.doc.length,
				insert: starter,
			},
			effects: languageCompartment.reconfigure(getLanguageExtension(ex.language)),
		});
	}
}

function renderFallbackExercise() {
	const titleEl = document.getElementById('exercise-title');
	const descEl = document.getElementById('exercise-desc');
	if (titleEl) titleEl.textContent = 'Belum Ada Latihan Khusus';
	if (descEl) {
		descEl.textContent =
			'Pelajaran ini saat ini belum memiliki latihan soal terdaftar. Anda dapat mencoba kode pada editor atau membuka Area Uji Bebas.';
	}
}

function setupCodeEditor() {
	const mount = document.getElementById('lesson-codemirror-mount');
	if (!mount || editorView) return;

	const defaultStarter = `# Tulis solusi Anda di sini\n`;
	const startState = EditorState.create({
		doc: defaultStarter,
		extensions: [
			basicSetup,
			oneDark,
			languageCompartment.of(python()),
			EditorView.lineWrapping,
		],
	});

	editorView = new EditorView({
		state: startState,
		parent: mount,
	});

	const resetBtn = document.getElementById('btn-editor-reset');
	resetBtn?.addEventListener('click', () => {
		if (!editorView) return;
		const initial = activeExercise?.starter_code || `# Tulis solusi Anda di sini\n`;
		editorView.dispatch({
			changes: {
				from: 0,
				to: editorView.state.doc.length,
				insert: initial,
			},
		});
	});
}

function setupExecutionActions() {
	const btnRun = document.getElementById('btn-run-code') as HTMLButtonElement | null;
	const btnSubmit = document.getElementById('btn-submit-code') as HTMLButtonElement | null;
	const runLabel = document.getElementById('btn-run-label');
	const submitLabel = document.getElementById('btn-submit-label');
	const statusText = document.getElementById('execution-status-text');
	const consoleOutput = document.getElementById('eval-console-output');
	const evalModeLabel = document.getElementById('eval-mode-label');
	const metricTime = document.getElementById('metric-exec-time');
	const metricStatus = document.getElementById('metric-exec-status');

	// RUN CODE (Free execution without test case evaluation)
	btnRun?.addEventListener('click', async () => {
		if (!editorView || btnRun.disabled) return;
		const code = editorView.state.doc.toString();
		const language = activeExercise?.language || 'python';

		setRunningState(true, 'run');
		if (consoleOutput) {
			consoleOutput.innerHTML = `<div class="text-[#facc15] animate-pulse">[UJI COBA KODE] Menjalankan kode di container sandbox...</div>`;
		}

		const startTime = performance.now();
		const res = await runPlaygroundCode(code, language);
		const elapsed = Math.round(performance.now() - startTime);

		setRunningState(false, 'run');

		if (evalModeLabel) evalModeLabel.textContent = 'MODE: RUN (UJI BEBAS)';
		if (metricTime) metricTime.textContent = `${elapsed}ms`;

		if (!res.ok) {
			if (metricStatus) {
				metricStatus.textContent = 'Gagal';
				metricStatus.className = 'text-[#ffb4ab]';
			}
			if (consoleOutput) {
				consoleOutput.innerHTML = `
					<div class="text-[#ffb4ab] font-bold">[GAGAL MENJALANKAN KODE]</div>
					<div class="text-[#ffdad6] whitespace-pre-wrap">${escapeHtml(res.error || 'Terjadi kesalahan eksekusi.')}</div>
				`;
			}
			return;
		}

		const data = res.data!;
		const hasError = !data.success || Boolean(data.stderr);
		if (metricStatus) {
			metricStatus.textContent = hasError ? 'Error Output' : 'Sukses';
			metricStatus.className = hasError ? 'text-[#ffb4ab]' : 'text-[#4edea3]';
		}

		if (consoleOutput) {
			let html = ``;
			if (data.stdout) {
				html += `<div class="text-[#e4e1e6] whitespace-pre-wrap leading-relaxed">${escapeHtml(data.stdout)}</div>`;
			}
			if (data.stderr) {
				html += `<div class="text-[#ffb4ab] whitespace-pre-wrap mt-2">${escapeHtml(data.stderr)}</div>`;
			}
			if (!data.stdout && !data.stderr) {
				html += `<div class="text-[#9a9078]">[Eksekusi selesai tanpa output stdout/stderr]</div>`;
			}
			consoleOutput.innerHTML = html;
		}
	});

	// SUBMIT CODE (All-or-nothing test case evaluation)
	btnSubmit?.addEventListener('click', async () => {
		if (!editorView || btnSubmit.disabled) return;

		if (!activeExercise) {
			alert('Latihan belum siap untuk disubmit.');
			return;
		}

		if (!isAuthenticated()) {
			if (consoleOutput) {
				consoleOutput.innerHTML = `
					<div class="bg-[#1f170b] border-2 border-[#facc15] p-3 text-[#ffecb9]">
						<div class="font-bold flex items-center gap-1.5 mb-1">
							<span class="material-symbols-outlined text-[16px]">lock</span>
							LOGIN DIPERLUKAN
						</div>
						<p>Anda perlu masuk (login) terlebih dahulu agar progres kelulusan latihan dan XP dapat dicatat secara permanen.</p>
						<a href="/login" class="inline-block mt-2 font-bold underline text-[#facc15]">[ Masuk Sekarang &rarr; ]</a>
					</div>
				`;
			}
			return;
		}

		const code = editorView.state.doc.toString();
		const language = activeExercise.language || 'python';
		const token = getAccessToken();

		setRunningState(true, 'submit');
		if (consoleOutput) {
			consoleOutput.innerHTML = `
				<div class="text-[#facc15] animate-pulse flex items-center gap-2">
					<span class="material-symbols-outlined text-[16px] animate-spin">sync</span>
					[PENILAIAN OTOMATIS] Menguji solusi terhadap seluruh test case...
				</div>
			`;
		}

		const startTime = performance.now();
		const res = await submitExerciseCode(activeExercise.id, code, language, token);
		const elapsed = Math.round(performance.now() - startTime);

		setRunningState(false, 'submit');

		if (evalModeLabel) evalModeLabel.textContent = 'MODE: SUBMISSION (EVALUASI TEST CASES)';
		if (metricTime) metricTime.textContent = `${elapsed}ms`;

		if (!res.ok) {
			if (metricStatus) {
				metricStatus.textContent = 'Gagal';
				metricStatus.className = 'text-[#ffb4ab]';
			}
			if (consoleOutput) {
				consoleOutput.innerHTML = `
					<div class="text-[#ffb4ab] font-bold">[GAGAL MENGEVALUASI SUBMISSION]</div>
					<div class="text-[#ffdad6] whitespace-pre-wrap">${escapeHtml(res.error || 'Terjadi kesalahan sistem.')}</div>
				`;
			}
			return;
		}

		const data = res.data!;
		const isPassed = data.success === true;

		if (metricStatus) {
			metricStatus.textContent = isPassed ? 'PASSED (100%)' : 'FAILED';
			metricStatus.className = isPassed ? 'text-[#4edea3]' : 'text-[#ffb4ab]';
		}

		if (consoleOutput) {
			if (isPassed) {
				consoleOutput.innerHTML = `
					<div class="bg-[#003824] border-2 border-[#4edea3] p-4 text-[#e4e1e6] flex flex-col gap-2 shadow-[4px_4px_0px_#000000]">
						<div class="flex items-center gap-2 text-[#4edea3] font-bold text-sm">
							<span class="material-symbols-outlined text-[20px]">verified</span>
							<span>SELAMAT! SOLUSI DITERIMA (PASSED)</span>
						</div>
						<p class="text-xs text-[#d1c6ab]">
							Seluruh test case berhasil dipenuhi dengan sempurna (100% lulus).
						</p>
						<div class="flex items-center gap-3 pt-1 border-t border-[#005234] text-xs">
							<span class="text-[#facc15] font-bold">+${activeExercise.xp_reward || 100} XP Ditambahkan</span>
							<span class="text-[#9a9078]">|</span>
							<span>Durasi Eksekusi: ${data.duration_ms || elapsed}ms</span>
						</div>
					</div>
				`;
			} else {
				consoleOutput.innerHTML = `
					<div class="bg-[#1F080A] border-2 border-[#ffb4ab] p-4 text-[#e4e1e6] flex flex-col gap-2 shadow-[4px_4px_0px_#93000a]">
						<div class="flex items-center gap-2 text-[#ffb4ab] font-bold text-sm">
							<span class="material-symbols-outlined text-[20px]">cancel</span>
							<span>SOLUSI BELUM MEMENUHI (FAILED)</span>
						</div>
						<p class="text-xs text-[#ffdad6]">
							Kode Anda tidak menghasilkan output yang diharapkan pada test case yang diuji. Evaluasi bersifat biner (All-or-Nothing).
						</p>
						${
							data.stderr
								? `<div class="bg-[#131316] p-2 border border-[#ffb4ab]/40 text-[#ffb4ab] text-xs font-mono whitespace-pre-wrap">${escapeHtml(data.stderr)}</div>`
								: ''
						}
						${
							data.stdout
								? `<div class="text-[#d1c6ab] text-xs pt-1">Output aktual terakhir: <code class="text-white">${escapeHtml(data.stdout)}</code></div>`
								: ''
						}
					</div>
				`;
			}
		}
	});

	function setRunningState(isRunning: boolean, type: 'run' | 'submit') {
		if (btnRun) btnRun.disabled = isRunning;
		if (btnSubmit) btnSubmit.disabled = isRunning;

		if (isRunning) {
			if (type === 'run' && runLabel) runLabel.textContent = 'Mengeksekusi...';
			if (type === 'submit' && submitLabel) submitLabel.textContent = 'Menilai...';
			if (statusText) statusText.textContent = 'Sedang Berjalan...';
		} else {
			if (runLabel) runLabel.textContent = 'Uji Coba Kode (Run)';
			if (submitLabel) submitLabel.textContent = 'Kirim Solusi (Submit)';
			if (statusText) statusText.textContent = 'Siap';
		}
	}
}
