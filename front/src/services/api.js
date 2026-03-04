import axios from 'axios';

const API_URL = import.meta.env.VITE_API_BASE_URL ?? '/api';

const api = axios.create({
    baseURL: API_URL,
    headers: {
        'Content-Type': 'application/json',
    },

})


api.interceptors.request.use(
    (config) => {
        if (config.data instanceof FormData) {
            delete config.headers['Content-Type'];
        }
        return config;
    }, 
    (error) => {
        return Promise.reject(error);
    },
)

api.interceptors.response.use(
    (response) => {
        return response;
    },
    (error) => {
        const originalRequest = error.config;

        if (error.response && error.response.status === 401 && !originalRequest._retry) {
            originalRequest._retry = true;
            return Promise.reject(error);
        }
        return Promise.reject(error);
    }
)
export default {
    get: (endpoint, config = {}) => api.get(endpoint, config),
    post: (endpoint, data, config = {}) => api.post(endpoint, data, config),
    put: (endpoint, data, config = {}) => api.put(endpoint, data, config),
    delete: (endpoint, config = {}) => api.delete(endpoint, config),
}