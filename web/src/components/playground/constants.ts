// web/src/components/playground/constants.ts
import type { SupportedLanguage, LanguageConfig } from './types';

export const LANGUAGES: Record<SupportedLanguage, LanguageConfig> = {
	python: {
		id: 'python',
		label: 'Python 3.11',
		emoji: '🐍',
		filename: 'main.py',
		version: 'Aktif',
		commandName: 'python3 main.py',
		starterCode: `# KODINGAN Playground v0.1.0
# Silakan eksperimen koding bebas di sini

def halo():
    nama_platform = "KODINGAN"
    pesan = "Selamat datang di KODINGAN!"
    print(pesan)
    print(f"Mari belajar dan eksplorasi di platform {nama_platform}.")

halo()

# Kalkulasi deret kuadrat
kuadrat = [x ** 2 for x in range(1, 6)]
print(f"Deret Kuadrat: {kuadrat}")
`,
	},
	javascript: {
		id: 'javascript',
		label: 'JavaScript (Node.js)',
		emoji: '⚡',
		filename: 'index.js',
		version: 'v20.x',
		commandName: 'node index.js',
		starterCode: `// KODINGAN Playground v0.1.0
function halo() {
    const namaPlatform = "KODINGAN";
    const pesan = "Selamat datang di KODINGAN!";
    console.log(pesan);
    console.log(\`Mari belajar dan eksplorasi di platform \${namaPlatform}.\`);
}

halo();

const kuadrat = [1, 2, 3, 4, 5].map(x => x ** 2);
console.log(\`Deret Kuadrat: [\${kuadrat.join(", ")}]\`);
`,
	},
	rust: {
		id: 'rust',
		label: 'Rust (Cargo)',
		emoji: '🦀',
		filename: 'main.rs',
		version: 'v1.75',
		commandName: 'cargo run --release',
		starterCode: `// KODINGAN Playground v0.1.0
fn main() {
    let nama_platform = "KODINGAN";
    let pesan = "Selamat datang di KODINGAN!";
    println!("{}", pesan);
    println!("Mari belajar dan eksplorasi di platform {}.", nama_platform);

    let kuadrat: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("Deret Kuadrat: {:?}", kuadrat);
}
`,
	},
};
