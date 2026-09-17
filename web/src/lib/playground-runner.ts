// web/src/lib/playground-runner.ts
import { EditorView, basicSetup } from 'codemirror';
import { EditorState, Compartment } from '@codemirror/state';
import { python } from '@codemirror/lang-python';
import { javascript } from '@codemirror/lang-javascript';
import { rust } from '@codemirror/lang-rust';
import { oneDark } from '@codemirror/theme-one-dark';
import { LANGUAGES } from '@/components/playground/constants';
import type { SupportedLanguage } from '@/components/playground/types';
import { getCurrentUser } from '@/lib/auth';
import { isFeatureEnabled, getFeatureFlags } from '@/lib/feature-flags';

function getLanguageExtension(lang: SupportedLanguage) {
	switch (lang) {
		case 'javascript':
			return javascript();
		case 'rust':
			return rust();
		case 'python':
		default:
			return python();
	}
}

function escapeHtml(str: string): string {
	return str
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&#039;');
}

export function initPlayground() {
	const mountPoint = document.getElementById('codemirror-mount');
	if (!mountPoint) return;

	const langBtn = document.getElementById('lang-btn');
	const langDropdown = document.getElementById('lang-dropdown');
	const selectedEmoji = document.getElementById('selected-lang-emoji');
	const selectedLabel = document.getElementById('selected-lang-label');
	const tabFilename = document.getElementById('tab-filename');
	const btnRun = document.getElementById('btn-run') as HTMLButtonElement | null;
	const runLabel = document.getElementById('run-label');
	const runIcon = document.getElementById('run-icon');
	const btnClear = document.getElementById('btn-clear');
	const btnReset = document.getElementById('btn-reset');
	const btnCopyLogs = document.getElementById('btn-copy-logs');
	const statusText = document.getElementById('status-text');
	const statusDot = document.getElementById('status-dot');
	const logFeed = document.getElementById('log-feed');
	const terminalBody = document.getElementById('terminal-body');
	const metricTime = document.getElementById('metric-time');
	const metricMem = document.getElementById('metric-mem');
	const metricExit = document.getElementById('metric-exit');
	const metricExitIcon = document.getElementById('metric-exit-icon');
	const editorLinesMetric = document.getElementById('editor-lines-metric');
	const editorCharsMetric = document.getElementById('editor-chars-metric');

	let currentLang: SupportedLanguage = 'python';
	const languageCompartment = new Compartment();

	// Inisialisasi CodeMirror 6 View
	const state = EditorState.create({
		doc: LANGUAGES[currentLang].starterCode,
		extensions: [
			basicSetup,
			oneDark,
			languageCompartment.of(getLanguageExtension(currentLang)),
			EditorView.lineWrapping,
			EditorView.updateListener.of((update) => {
				if (update.docChanged) {
					updateEditorMetrics();
				}
			}),
		],
	});

	const editorView = new EditorView({
		state,
		parent: mountPoint,
	});

	function updateEditorMetrics() {
		const doc = editorView.state.doc;
		if (editorLinesMetric) editorLinesMetric.textContent = `BARIS: ${String(doc.lines).padStart(2, '0')}`;
		if (editorCharsMetric) editorCharsMetric.textContent = `PANJANG: ${doc.length} KARAKTER`;
	}

	updateEditorMetrics();

	async function updateLanguageFeatureStatus(skipStatusText = false) {
		try {
			const isCurrentActive = await isFeatureEnabled(`runner:${currentLang}`);
			if (!isCurrentActive) {
				if (btnRun) {
					btnRun.disabled = true;
					if (runLabel) runLabel.textContent = 'Runner Dalam Perbaikan';
				}
				if (!skipStatusText && statusText && statusDot) {
					statusText.textContent = '● RUNNER NON-AKTIF';
					statusText.className = 'font-mono text-xs text-[#ffb4ab] tracking-wider font-bold';
					statusDot.className = 'w-2.5 h-2.5 bg-[#ffb4ab]';
				}
			} else {
				if (btnRun) {
					btnRun.disabled = false;
					if (runLabel) runLabel.textContent = 'Jalankan Kode';
				}
				if (!skipStatusText && statusText && statusDot) {
					statusText.textContent = '● SIAP';
					statusText.className = 'font-mono text-xs text-[#4edea3] tracking-wider font-bold';
					statusDot.className = 'w-2.5 h-2.5 bg-[#4edea3] animate-pulse';
				}
			}

			if (langDropdown) {
				const buttons = langDropdown.querySelectorAll('button');
				for (const btn of Array.from(buttons)) {
					const lang = btn.getAttribute('data-lang');
					if (lang) {
						const active = await isFeatureEnabled(`runner:${lang}`);
						const badge = btn.querySelector('span:last-child');
						if (badge) {
							if (!active) {
								badge.textContent = '[Perbaikan]';
								badge.className = 'text-[#ffb4ab] text-[11px] font-bold';
							} else {
								badge.textContent = lang === 'python' ? 'Aktif' : (LANGUAGES[lang as SupportedLanguage]?.version || 'Aktif');
								badge.className = 'text-[#4edea3] text-[11px]';
							}
						}
					}
				}
			}
		} catch (err) {
			console.warn('Gagal sinkronisasi status runner bahasa:', err);
		}
	}

	updateLanguageFeatureStatus();

	// Dropdown Handler
	if (langBtn && langDropdown) {
		langBtn.addEventListener('click', (e) => {
			e.stopPropagation();
			langDropdown.classList.toggle('hidden');
			langDropdown.classList.toggle('flex');
		});

		document.addEventListener('click', (e) => {
			if (!document.getElementById('lang-menu-wrapper')?.contains(e.target as Node)) {
				langDropdown.classList.add('hidden');
				langDropdown.classList.remove('flex');
			}
		});

		langDropdown.querySelectorAll('button').forEach((btn) => {
			btn.addEventListener('click', () => {
				const lang = (btn.getAttribute('data-lang') || 'python') as SupportedLanguage;
				const config = LANGUAGES[lang];
				if (!config) return;

				currentLang = lang;
				if (selectedEmoji) selectedEmoji.textContent = config.emoji;
				if (selectedLabel) selectedLabel.textContent = config.label;
				if (tabFilename) tabFilename.textContent = config.filename;

				langDropdown.classList.add('hidden');
				langDropdown.classList.remove('flex');

				editorView.dispatch({
					effects: languageCompartment.reconfigure(getLanguageExtension(lang)),
					changes: {
						from: 0,
						to: editorView.state.doc.length,
						insert: config.starterCode,
					},
				});

				if (statusText && statusDot) {
					statusText.textContent = '● RECONFIG';
					statusText.className = 'font-mono text-xs text-[#facc15] tracking-wider font-bold';
					statusDot.className = 'w-2.5 h-2.5 bg-[#facc15] animate-pulse';

					setTimeout(() => {
						statusText.textContent = '● SIAP';
						statusText.className = 'font-mono text-xs text-[#4edea3] tracking-wider font-bold';
						statusDot.className = 'w-2.5 h-2.5 bg-[#4edea3] animate-pulse';
					}, 350);
				}

				updateLanguageFeatureStatus();
			});
		});
	}

	// Reset Code Handler
	btnReset?.addEventListener('click', () => {
		editorView.dispatch({
			changes: {
				from: 0,
				to: editorView.state.doc.length,
				insert: LANGUAGES[currentLang].starterCode,
			},
		});

		if (statusText) {
			const prev = statusText.textContent;
			statusText.textContent = '● RESET SELESAI';
			setTimeout(() => {
				statusText.textContent = prev;
			}, 600);
		}
  });



	// Clear Console Handler
  btnClear?.addEventListener('click', () => {
    function getUsername() {
        const user = getCurrentUser();
        const usernameEl = document.getElementById("terminal-prompt");
        if (user && usernameEl) {
            usernameEl.textContent = `${user.username}@kodingan:~$`;
        } else {
            if (usernameEl) {
                usernameEl.textContent = `guest@kodingan:~$`;
            }
        }
    }
		if (logFeed) {
			logFeed.innerHTML = `
				<div class="text-[#d1c6ab] pb-2 border-b border-[#353438] text-xs">
					<div>Terminal dibersihkan oleh sinyal pengguna.</div>
				</div>
				<div class="flex items-center gap-2 pt-1" id="terminal-prompt-line">
					<span class="text-[#4edea3] font-bold select-none" id="terminal-prompt"></span>
					<span class="w-2 h-4 bg-[#4edea3] inline-block animate-pulse"></span>
				</div>
			`;
    }
		getUsername();
	});

	// Copy Logs Handler
	btnCopyLogs?.addEventListener('click', () => {
		if (terminalBody) {
			navigator.clipboard.writeText(terminalBody.innerText);
			if (statusText) {
				const prev = statusText.textContent;
				statusText.textContent = '● LOG TERSALIN!';
				setTimeout(() => {
					statusText.textContent = prev;
				}, 700);
			}
		}
	});

	// Run Code Handler
	async function runCode() {
		if (!btnRun || btnRun.disabled) return;

		const code = editorView.state.doc.toString();
		const language = currentLang;

		btnRun.disabled = true;
		if (runLabel) runLabel.textContent = 'Mengeksekusi...';
		if (runIcon) runIcon.textContent = 'hourglass_top';

		if (statusText && statusDot) {
			statusText.textContent = '● MENGEKSEKUSI...';
			statusText.className = 'font-mono text-xs text-[#facc15] tracking-wider font-bold';
			statusDot.className = 'w-2.5 h-2.5 bg-[#facc15] animate-ping';
		}

		const startTime = performance.now();

		try {
			const response = await fetch('/api/v1/playground/run', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify({
					language,
					code,
				}),
			});

			const json = await response.json().catch(() => ({}));
			const duration = Math.round(performance.now() - startTime);

			if (!response.ok) {
				const errMsg = json.error?.message || `HTTP ${response.status}: Eksekusi gagal`;
				const isMaintenance = response.status === 503;

				if (isMaintenance) {
					await getFeatureFlags(true);
					await updateLanguageFeatureStatus();
				}

				if (statusText && statusDot) {
					statusText.textContent = isMaintenance ? '● PEMELIHARAAN' : `● ERROR (${response.status})`;
					statusText.className = `font-mono text-xs ${isMaintenance ? 'text-[#facc15]' : 'text-[#ffb4ab]'} tracking-wider font-bold`;
					statusDot.className = `w-2.5 h-2.5 ${isMaintenance ? 'bg-[#facc15]' : 'bg-[#ffb4ab]'}`;
				}

				if (logFeed) {
					const errorEntry = document.createElement('div');
					errorEntry.className = isMaintenance
						? 'bg-[#1f170b] border-2 border-[#facc15] p-3 text-xs text-[#ffecb9] shadow-[3px_3px_0px_#000000] font-mono'
						: 'bg-[#1F080A] border-2 border-[#ffb4ab] p-3 text-xs text-[#ffb4ab] shadow-[2px_2px_0px_#93000a]';

					errorEntry.innerHTML = `
						<div class="font-bold uppercase flex items-center gap-1.5 pb-1 border-b ${isMaintenance ? 'border-[#facc15]/40 text-[#facc15]' : 'border-[#ffb4ab]/30'}">
							<span class="material-symbols-outlined text-[16px]">${isMaintenance ? 'build' : 'error'}</span>
							${isMaintenance ? '[STATUS: 503 FITUR DALAM PEMELIHARAAN]' : '[GAGAL MENGEKSEKUSI KODE]'}
						</div>
						<div class="pt-1.5 leading-relaxed">${escapeHtml(errMsg)}</div>
						${isMaintenance ? '<div class="text-[10px] text-[#9a9078] pt-1">Bahasa pemrograman ini sedang dinonaktifkan sementara di database.</div>' : ''}
					`;

					const promptLine = document.getElementById('terminal-prompt-line');
					if (promptLine) {
						logFeed.insertBefore(errorEntry, promptLine);
					} else {
						logFeed.appendChild(errorEntry);
					}

					if (terminalBody) {
						terminalBody.scrollTop = terminalBody.scrollHeight;
					}
				}
				return;
			}

			const result = json.data;
			const stdout = result.stdout || '';
			const stderr = result.stderr || '';
			const exitCode = result.exit_code ?? 0;
			const isSuccess = exitCode === 0 && !stderr;

			if (metricTime) metricTime.textContent = `${duration}ms`;
			if (metricMem) metricMem.textContent = `~14.5 MB`;
			if (metricExit) {
				metricExit.textContent = `${exitCode} (${isSuccess ? 'Sukses' : 'Gagal'})`;
				metricExit.className = isSuccess ? 'text-[#4edea3]' : 'text-[#ffb4ab]';
			}
			if (metricExitIcon) {
				metricExitIcon.textContent = isSuccess ? 'check_circle' : 'error';
				metricExitIcon.className = `material-symbols-outlined text-[15px] ${isSuccess ? 'text-[#4edea3]' : 'text-[#ffb4ab]'}`;
			}

			if (logFeed) {
				const entry = document.createElement('div');
				entry.className = 'pt-2 space-y-1 border-t border-[#353438]/50';

				const commandName = LANGUAGES[language].commandName;

				let outputHtml = '';
				if (stdout) {
					outputHtml += `<div class="text-[#e4e1e6] whitespace-pre-wrap">${escapeHtml(stdout)}</div>`;
				}
				if (stderr) {
					outputHtml += `
						<div class="bg-[#1F080A] border-2 border-[#ffb4ab] p-2 mt-1 shadow-[2px_2px_0px_#93000a] text-xs text-[#ffb4ab] whitespace-pre-wrap font-mono">
							<div class="font-bold uppercase pb-1 border-b border-[#ffb4ab]/30 flex items-center gap-1">
								<span class="material-symbols-outlined text-[14px]">warning</span>
								[STDERR EXCEPTION TRACE]
							</div>
							<div class="pt-1">${escapeHtml(stderr)}</div>
						</div>
					`;
				}
				if (!stdout && !stderr) {
					outputHtml = `<div class="text-[#d1c6ab] italic text-xs">(Eksekusi selesai tanpa output stdout)</div>`;
				}

				entry.innerHTML = `
					<div class="flex items-center gap-2 text-[#e4e1e6]">
						<span class="text-[#4edea3] font-bold select-none">guest@kodingan:~$</span>
						<span class="text-[#ffecb9] font-bold">${commandName}</span>
					</div>
					<div class="pl-2 border-l-2 border-[#353438] space-y-0.5">
						${outputHtml}
					</div>
				`;

				const promptLine = document.getElementById('terminal-prompt-line');
				if (promptLine) {
					logFeed.insertBefore(entry, promptLine);
				} else {
					logFeed.appendChild(entry);
				}

				if (terminalBody) {
					terminalBody.scrollTop = terminalBody.scrollHeight;
				}
			}

			if (statusText && statusDot) {
				if (isSuccess) {
					statusText.textContent = '● SUKSES (0)';
					statusText.className = 'font-mono text-xs text-[#4edea3] tracking-wider font-bold';
					statusDot.className = 'w-2.5 h-2.5 bg-[#4edea3]';
				} else {
					statusText.textContent = `● GAGAL (${exitCode})`;
					statusText.className = 'font-mono text-xs text-[#ffb4ab] tracking-wider font-bold';
					statusDot.className = 'w-2.5 h-2.5 bg-[#ffb4ab]';
				}
			}
		} catch (err: any) {
			if (statusText && statusDot) {
				statusText.textContent = '● ERROR KONEKSI';
				statusText.className = 'font-mono text-xs text-[#ffb4ab] tracking-wider font-bold';
				statusDot.className = 'w-2.5 h-2.5 bg-[#ffb4ab]';
			}

			if (logFeed) {
				const errorEntry = document.createElement('div');
				errorEntry.className = 'bg-[#1F080A] border-2 border-[#ffb4ab] p-3 text-xs text-[#ffb4ab] shadow-[2px_2px_0px_#93000a]';
				errorEntry.innerHTML = `
					<div class="font-bold uppercase flex items-center gap-1.5 pb-1 border-b border-[#ffb4ab]/30">
						<span class="material-symbols-outlined text-[16px]">error</span>
						[GAGAL MENGHUBUNGI SERVER]
					</div>
					<div class="pt-1.5">${escapeHtml(err.message || 'Pastikan backend Rust aktif di port 3000.')}</div>
				`;
				const promptLine = document.getElementById('terminal-prompt-line');
				if (promptLine) {
					logFeed.insertBefore(errorEntry, promptLine);
				} else {
					logFeed.appendChild(errorEntry);
				}
			}
		} finally {
			await updateLanguageFeatureStatus(true);
			if (runIcon) runIcon.textContent = 'play_arrow';
		}
	}

	btnRun?.addEventListener('click', runCode);

	document.addEventListener('keydown', (e) => {
		if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
			e.preventDefault();
			runCode();
		}
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'l') {
			e.preventDefault();
			btnClear?.click();
		}
	});
}
