import {resolve} from 'path'
import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react';
import dts from 'unplugin-dts/vite';

export default defineConfig({
    plugins: [
        react(),
        dts({insertTypesEntry: true, bundleTypes: true, copyDtsFiles: true}),
    ],
    build: {
        lib: {
            entry: resolve(__dirname, 'src/index.tsx'),
            name: '@phonologic/viewer',
            formats: ['es', 'cjs'],
            fileName: (format) => `index.${format}.js`
        },
        rolldownOptions: {
            external: ['react', "react/jsx-runtime", "bootstrap", "bootstrap-react"],
            output: {
                globals: {
                    react: 'React',
                    'react-dom': 'ReactDOM',
                    "react/jsx-runtime": "JSX",
                },
            },
        },
    },
})
