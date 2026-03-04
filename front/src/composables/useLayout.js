import { computed, ref } from 'vue';

const LOCAL_KEY = 'sockless:darkMode';

const appState = ref({
    darkMode: false
})

let initialDarkModeApplied = false;
export function useLayout() {
    function persist(value) {
        try {
            localStorage.setItem(LOCAL_KEY, value ? 'true' : 'false');
        } catch (e) {
            // localStorage can fail in some environments we ignore silently
        }
    }

    function setDarkMode(value) {
        appState.value.darkMode = value;
        if (value) {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');   
        }
        persist(value);
    } 

    function toggleDarkMode() {
        appState.value.darkMode = !appState.value.darkMode;
        document.documentElement.classList.toggle('dark');
        persist(appState.value.darkMode);
    }

    const isDarkMode = computed(() => appState.value.darkMode);

    if (!initialDarkModeApplied) {
        try {
            const saved = localStorage.getItem(LOCAL_KEY);
            if (saved === 'true' || saved === 'false') {
                setDarkMode(saved === 'true');
            } else {
                setDarkMode(appState.value.darkMode);
            }
        } catch (e) {
            // localStorage unavailable -> just apply default
            setDarkMode(appState.value.darkMode);
        }
        initialDarkModeApplied = true;
    }

    return {
        isDarkMode,
        setDarkMode,
        toggleDarkMode
    }
}
