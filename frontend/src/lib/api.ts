const BASE_URL = '/api';

export async function fetchApi(endpoint: string, options: RequestInit = {}) {
	const response = await fetch(`${BASE_URL}${endpoint}`, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options.headers
		}
	});

	return response;
}
