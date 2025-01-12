let userId = '';

export function setUserId(id: string) {
	userId = id;
}

export function clearUserId() {
	userId = '';
}

export async function fetchApi(endpoint: string, options: RequestInit = {}) {
	const headers = {
		...options.headers,
		...(userId && { 'X-User-ID': userId })
	};

	const response = await fetch(`/api${endpoint}`, {
		...options,
		headers
	});

	if (!response.ok) {
		throw new Error(`API call failed: ${response.statusText}`);
	}

	return response.json();
}
