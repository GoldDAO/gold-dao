module.exports = {
  env: {
    browser: true,
    commonjs: true,
    node: true,
    jest: true,
    es2021: true,
  },
  extends: ['airbnb-base', 'plugin:react/recommended', 'next/core-web-vitals'],
  parserOptions: {
    ecmaVersion: 'latest',
    ecmaFeatures: {
      jsx: true,
      modules: true,
    },
    sourceType: 'module',
  },
  rules: {
    'linebreak-style': 0,
    'react/prop-types': 0,
    'react/react-in-jsx-scope': 'off',
    'no-param-reassign': [
      2,
      {
        props: false,
      },
    ],
    'import/no-extraneous-dependencies': [
      'error',
      {
        devDependencies: ['.storybook/', '!*.stories.js'],
      },
    ],
  },
};
