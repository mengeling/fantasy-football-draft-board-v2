const BASE_URL = 'http://localhost:8080';

export async function fetchApi(endpoint: string, options: RequestInit = {}) {
	const response = await fetch(`${BASE_URL}${endpoint}`, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options.headers
		}
	});

	if (!response.ok) {
		throw new Error(`API call failed: ${response.statusText}`);
	}

	return response.json();
}
