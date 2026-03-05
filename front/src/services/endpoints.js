import api from './api';

export const endpoints = {
	shipmentTracking: (guideNum) => `/shipment/${encodeURIComponent(guideNum)}`,
	supportQuestions: '/employee/questions',
	answeredSupportQuestions: '/employee/questions/answered',
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

export async function createSupportQuestion(clientCid, questionText) {
	const normalizedQuestion = questionText?.trim();

	if (!Number.isInteger(clientCid) || clientCid <= 0) {
		throw new Error('Client ID must be a valid positive number.');
	}

	if (!normalizedQuestion) {
		throw new Error('Question text is required.');
	}

	try {
		const response = await api.post(endpoints.supportQuestions, {
			client_cid: clientCid,
			question_text: normalizedQuestion,
		});
		return response.data;
	} catch (error) {
		throw new Error(error?.response?.data?.message || 'Unable to create support question right now.');
	}
}

export async function getAnsweredSupportQuestions() {
	try {
		const response = await api.get(endpoints.answeredSupportQuestions);
		return Array.isArray(response.data) ? response.data : [];
	} catch (error) {
		throw new Error(error?.response?.data?.message || 'Unable to fetch answered questions right now.');
	}
}
