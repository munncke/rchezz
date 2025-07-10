// noinspection JSCheckFunctionSignatures

import eslint from '@eslint/js';
import prettierConfig from 'eslint-config-prettier';
import tseslint from 'typescript-eslint';

export default tseslint.config(
	eslint.configs.recommended,
	tseslint.configs.strictTypeChecked,
	prettierConfig,
	{
		languageOptions: {
			parserOptions: {
				projectService: true,
				tsconfigRootDir: import.meta.dirName,
			},
		},
	},
	tseslint.configs.stylisticTypeChecked
);
