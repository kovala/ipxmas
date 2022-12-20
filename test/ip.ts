import {$} from "https://deno.land/x/deno_dx/mod.ts";
import {chain, keys, replace, trim, times} from "https://cdn.skypack.dev/lodash";
import {parse} from "https://deno.land/std@0.159.0/flags/mod.ts";

const url = `http://localhost:8668`
const log = console.log;

export const runMap = async (map) => {
	const flags = parse(Deno.args);
	delete flags['_']
	const [key] = keys(flags);
	const fn = map[key];
	if (fn){
		await fn();
	}
}
const rndIp = () => {
	const oct = ()=> Math.floor(Math.random()*255)
	return `${oct()+1}.${oct()}.${oct()}.${oct()}`;
}


const mem = async () => {
	const a = await $`ps -eo pid,cmd | grep rooste | head -1`;
	const [pid0] = chain(a).trim().split(/\s+/).value();
	const pid = trim(pid0);

	const b = await $`sudo pmap ${pid} | grep total`;
	const [_, c] = chain(b).trim().split(/\s+/).value();

	const m = +replace(c, 'K', '')/1024**2;
	log(`%c${m.toFixed(2)}GB`, "background-color:blue")
}
const refresh = async () => {
	const r = await fetch(`${url}/refresh`);
	log(await r.text());
}
const ips = async (verb=true) => {
	const value = chain(times(100)).map(i=>rndIp()).join(',').value();
	const path = `${url}/ips?value=${value}`;
	verb && log(`%c${path}`, "background-color:blue")

	const r = await fetch(path);
	const t= await r.text();
	verb && log(`%c${t}`, "background-color:indigo")
}
const ips10 = async (times = 10) => {
	const t0 = performance.now();

	for (let i = 0; i < times; ++i) {
		await ips(true);
	}

	const t1 = performance.now();
	const elapsed = (t1-t0)/times;
	log(`%c avg request time: ${elapsed.toFixed(2)}ms`, "background-color:darkblue;color:yellow")
}
const score = () => {
	const refreshTime = `877.224495ms`;

	const initialGb = 0.47;
	const reloadGb = 1.74;
	const mem1st = reloadGb-initialGb;

	const avg100x10 = `1.75ms`

  log(`mem: ${mem1st}`)
}

await runMap({
	refresh, ips,	ips10,
	mem, score
})