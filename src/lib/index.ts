// place files you want to import through the `$lib` alias in this folder.
import { invoke } from '@tauri-apps/api/tauri';
import { ask, message } from '@tauri-apps/api/dialog';
import { Store } from 'tauri-plugin-store-api';
import { goto, invalidateAll } from '$app/navigation';

// TODO:
//  + Error handling
// 	+ Reset params
// 		- lock account
// 		- add keys
// 			* ASH

const st = new Store('');

type User = {
	username: string;
	pwhash: string;
	uid: string;
	kraken_auth: boolean;
	ash_auth: boolean;
	sensitive: string;
};

type SensitiveParams = {
	email: string;
	kraken_key: string | null;
	kraken_sk: string | null;
	ash_key: string | null;
	ash_sk: string | null;
};

type Balance = {
	name: string;
	amount: number;
};

type TradingPair = {
	exchange_name: string;
	base: string;
	quote: string;
	price: number;
	high: number;
	delta: number;
	base_decimals: number;
	quote_decimals: number;
	ordermin: number;
	costmin: number;
	tick_size: number;
};

type Asset = {
	name: string;
	balance: number;
	valueUsd: number;
	valueBtc: number;
	priceUsd: number;
	priceBtc: number;
};

type Transaction = {
	txid: string;
	refid: string;
	time: number;
	type_field: string;
	asset: string;
	amount: number;
	fee: number;
	balance: number;
};

export async function viewMarket(exchangeName: string): Promise<null> {
	await st.set('market', null);
	await st.set('market', exchangeName);
	goto('/', { invalidateAll: true }).then(() => goto('/market', { invalidateAll: true }));
	return null;
}

export async function getSelectedMarket(): Promise<string> {
	const market: string | null = await st.get('market');
	if (market) {
		await st.set('market', null);
		return market;
	}
	return 'XXBTZUSD';
}

export async function addOrder(
	quote: string,
	base: string,
	side: string,
	amt: number
): Promise<null | string> {
	const s: string = `${side} ${amt} ${base}/${quote} `;
	console.log(s);
	const params: SensitiveParams | null = await st.get('params');
	if (params) {
		const k = params.kraken_key;
		const sk = params.kraken_sk;
		try {
			await invoke('submit_order', { krakenApi: k, krakenSk: sk, quote, base, side, amt });
			goto('/portfolio').then(() => goto('/portfolio'));
			await ok('Transaction submitted successfully');
			return null;
		} catch (e) {
			let errMsg: string;
			console.log(e);
			if (typeof e === 'string') {
				errMsg = e.toUpperCase();
			} else if (e instanceof Error) {
				errMsg = e.message;
			} else {
				errMsg = 'an error occured with the transaction';
			}
			await err(errMsg);
		}
	}
	return 'Account locked';
}

export async function unlocked(): Promise<boolean> {
	const params: SensitiveParams | null = await st.get('params');
	if (params) {
		return true;
	}
	return false;
}

export async function getUser(): Promise<User | null> {
	let user: User | null = await st.get('user');
	if (!user) {
		try {
			user = await invoke('get_user');
			await storeUser(user);
		} catch {
			user = null;
		}
	}
	return user;
}

export async function storeUser(user: User | null) {
	if (user) {
		await st.set('user', user);
	}
}

export async function storeParams(params: SensitiveParams | null) {
	if (params) {
		await st.set('params', params);
	}
}

export async function logout() {
	const confirm = await ask('Are you sure you want to logout?', {
		title: 'ASH Trader',
		type: 'warning'
	});
	if (confirm) {
		await clearUserStore();
		await invoke('logout');
		goto('/').then(() => goto('/'));
	}
}

export async function getBalances(): Promise<Balance[] | null> {
	const params: SensitiveParams | null = await st.get('params');
	if (params) {
		const k = params.kraken_key;
		const sk = params.kraken_sk;
		try {
			const balances: Balance[] = await invoke('get_balances', { krakenApi: k, krakenSk: sk });
			return balances;
		} catch (e) {
			return [];
		}
	}
	return null;
}

export async function getTradingPairs(): Promise<TradingPair[] | null> {
	try {
		const pairs: TradingPair[] = await invoke('get_trading_pairs');
		return pairs;
	} catch (e) {
		console.log(e);
		return null;
	}
}

export async function getLedger(): Promise<Transaction[] | null> {
	const params: SensitiveParams | null = await st.get('params');
	if (params) {
		const k = params.kraken_key;
		const sk = params.kraken_sk;
		try {
			let ledger: Transaction[] = await invoke('get_ledger', { krakenApi: k, krakenSk: sk });
			ledger.forEach((t) => {
				const trans = ledger.find((tx) => tx.refid == t.refid);
				if (trans) {
					t.time = trans.time;
				}
			});
			ledger = ledger.sort((a, b) => {
				if (a.time > b.time) {
					return -1;
				}
				if (a.time < b.time) {
					return 1;
				}
				if (a.time == b.time) {
					if (a.amount > b.amount) {
						return -1;
					}
					if (a.amount < b.amount) {
						return 1;
					}
				}
				return 0;
			});
			return ledger;
		} catch (e) {
			console.log(e);
		}
	}
	return null;
}

export async function getPortfolio(): Promise<Asset[] | null> {
	const params: SensitiveParams | null = await st.get('params');
	if (params) {
		const k = params.kraken_key;
		const sk = params.kraken_sk;
		try {
			const balances: Balance[] = await invoke('get_balances', { krakenApi: k, krakenSk: sk });
			const pairs: TradingPair[] = await invoke('get_trading_pairs');
			let assets: Asset[] = [];
			balances.forEach((b) => {
				const prices = getPrices(b.name, pairs);
				const asset: Asset = {
					name: b.name,
					balance: b.amount,
					valueUsd: b.amount * prices[0],
					valueBtc: b.amount * prices[1],
					priceUsd: prices[0],
					priceBtc: prices[1]
				};
				assets.push(asset);
			});
			assets = assets.sort((a, b) => {
				if (a.valueBtc > b.valueBtc) {
					return -1;
				}
				if (a.valueBtc < b.valueBtc) {
					return 1;
				}
				return 0;
			});
			return assets;
		} catch (e) {
			console.log(e);
		}
	}
	return null;
}

export async function clearUserStore() {
	await st.set('user', null);
	await st.set('params', null);
}

export async function ok(msg: string) {
	if (msg) {
		await message(msg, 'ASH Trader');
	}
}

export async function err(msg: string) {
	if (msg) {
		await message(msg, { title: 'ASH Trader: ERROR', type: 'error' });
	}
}

export function validEmail(email: string): boolean {
	const x: RegExpMatchArray | null = String(email)
		.toLowerCase()
		.match(
			/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
		);
	return x != null;
}

function getPrices(name: string, pairs: TradingPair[]): number[] {
	let priceUsd = 1;
	let priceBtc = 1;
	if (name == 'BTC') {
		const pairUsd = pairs.find((element) => element.base == name && element.quote == 'USD');
		if (pairUsd) {
			priceUsd = parseFloat(pairUsd.price.toFixed(2));
		}
	} else if (name == 'USD') {
		const tmpPrice = pairs.find(
			(element) => element.base == 'BTC' && element.quote == 'USD'
		)?.price;
		if (tmpPrice) {
			priceBtc = parseFloat((1 / tmpPrice).toFixed(8));
		}
	} else {
		const pairUsd = pairs.find((element) => element.base == name && element.quote == 'USD');
		if (pairUsd) {
			priceUsd = parseFloat(pairUsd.price.toFixed(pairUsd.quote_decimals));
		}
		const pairBtc = pairs.find((element) => element.base == name && element.quote == 'BTC');
		if (pairBtc) {
			priceBtc = parseFloat(pairBtc.price.toFixed(pairBtc.quote_decimals));
		} else {
			const btcUsd = pairs.find(
				(element) => element.base == 'BTC' && element.quote == 'USD'
			)?.price;
			if (btcUsd) {
				priceBtc = parseFloat((priceUsd / btcUsd).toFixed(8));
			}
		}
	}
	return [priceUsd, priceBtc];
}
