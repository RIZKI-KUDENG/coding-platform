// web/src/components/playground/types.ts

export type SupportedLanguage = 'python' | 'javascript' | 'rust';

export interface LanguageConfig {
	id: SupportedLanguage;
	label: string;
	emoji: string;
	filename: string;
	version: string;
	starterCode: string;
	commandName: string;
}

export interface ExecutionMetric {
	time: string;
	mem: string;
	exitCode: number | null;
	isSuccess: boolean;
}

export interface TerminalLogEntry {
	id: string;
	commandName: string;
	stdout: string;
	stderr: string;
	exitCode: number;
	timestamp: number;
}
