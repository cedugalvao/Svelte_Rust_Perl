/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      backgroundImage: {
        'hero': "url('/assets/heroSvg.svg')",
      },
      height: {
        'screen-80': 'calc(100vh - 88px)',
      },
      textColor: {
        'laranja': '#F7AD19',
      },
    },
  },
  plugins: [],
}
