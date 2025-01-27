interface ApiOptions {
	userId?: string;
	method?: string;
	headers?: Record<string, string>;
	body?: string;
}

export async function fetchApi(endpoint: string, { userId, headers, ...options }: ApiOptions = {}) {
	const requestHeaders = {
		'Content-Type': 'application/json',
		...(userId && { 'X-User-ID': userId }),
		...headers
	};

	const response = await fetch(`/api${endpoint}`, {
		...options,
		headers: requestHeaders
	});

	if (!response.ok) {
		throw new Error(`API call failed: ${response.statusText}`);
	}

	return options.method === 'DELETE' ? null : response.json();
}
