/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./pages/**/*.html'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Space Grotesk', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      animation: {
        'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'spin-slow': 'spin 20s linear infinite',
        'morph-in': 'morphIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) forwards',
        'spring-reveal': 'springReveal 0.8s cubic-bezier(0.34, 1.56, 0.64, 1) forwards',
      },
      keyframes: {
        morphIn: {
          '0%': { opacity: '0', transform: 'scale(0.8) translateY(10px)' },
          '100%': { opacity: '1', transform: 'scale(1) translateY(0)' },
        },
        springReveal: {
          '0%': { opacity: '0', transform: 'translateY(20px) scale(0.95)' },
          '60%': { transform: 'translateY(-4px) scale(1.02)' },
          '100%': { opacity: '1', transform: 'translateY(0) scale(1)' },
        }
      }
    }
  },
  plugins: [],
}
