import init, { add } from '../pkg';

export async function run() {
	await init();
	console.log(add(BigInt(1), BigInt(3)));
}
