import api from './api';

export const endpoints = {
	shipmentTracking: (guideNum) => `/shipment/${encodeURIComponent(guideNum)}`,
};

function getTrackingErrorMessage(error) {
	const statusCode = error?.response?.status;

	if (statusCode === 404) {
		return 'Package code not found.';
	}

	if (statusCode === 400) {
		return 'Invalid package code.';
	}

	return error?.response?.data?.message || 'Unable to fetch package tracking right now.';
}

export async function getShipmentTracking(guideNum) {
	const normalizedGuide = guideNum?.trim();

	if (!normalizedGuide) {
		throw new Error('Package code is required.');
	}

	try {
		const response = await api.get(endpoints.shipmentTracking(normalizedGuide));
		return response.data;
	} catch (error) {
		throw new Error(getTrackingErrorMessage(error));
	}
}
