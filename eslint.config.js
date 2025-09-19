// ESLint Flat Config (ESLint 9+) - Vue 3 + TypeScript + Import hygiene + Prettier integration
// Migration: replaces previous eslint.config.ts to avoid requiring TS parsing for config.
import js from '@eslint/js';
import * as tseslint from 'typescript-eslint';
import vue from 'eslint-plugin-vue';
import importPlugin from 'eslint-plugin-import';
import unusedImports from 'eslint-plugin-unused-imports';
import globals from 'globals';
import vueParser from 'vue-eslint-parser';

// Type annotation: using root FlatConfig[] instead of deprecated Linter.FlatConfig[]
/** @type {import('eslint').FlatConfig[]} */
export default [
  {
    ignores: ['node_modules/**', 'dist/**', 'build/**', 'coverage/**', 'src-tauri/**'],
  },
  js.configs.recommended,
  ...vue.configs['flat/recommended'],
  ...tseslint.configs.recommended,
  importPlugin.flatConfigs.recommended,
  importPlugin.flatConfigs.typescript,
  {
    files: ['**/*.{ts,tsx,vue}'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        tsconfigRootDir: import.meta.dirname,
        projectService: true,
        extraFileExtensions: ['.vue'],
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: { ...globals.browser, ...globals.node },
    },
    plugins: { 'unused-imports': unusedImports, '@typescript-eslint': tseslint.plugin },
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-v-html': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/singleline-html-element-content-newline': 'off',
      'vue/html-self-closing': 'off',
      'vue/attributes-order': 'off',
      'vue/first-attribute-linebreak': 'off',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/no-empty-object-type': 'off',
      'unused-imports/no-unused-imports': 'warn',
      'import/no-unresolved': [
        'error',
        { ignore: ['^@/', '^@common/', '^@types/', '^@assets/', '^@components/'] },
      ],
      'import/order': [
        'warn',
        {
          'newlines-between': 'always',
          groups: [
            'builtin',
            'external',
            'internal',
            'parent',
            'sibling',
            'index',
            'object',
            'type',
          ],
          pathGroups: [{ pattern: '@/**', group: 'internal', position: 'after' }],
          alphabetize: { order: 'asc', caseInsensitive: true },
        },
      ],
      'no-console': ['warn', { allow: ['error', 'warn'] }],
      'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'warn',
    },
  },
  {
    files: ['**/*.config.*', 'scripts/**/*.*'],
    rules: { 'no-console': 'off' },
  },
  {
    files: ['src/shims-vue.d.ts'],
    rules: { '@typescript-eslint/no-explicit-any': 'off' },
  },
];
