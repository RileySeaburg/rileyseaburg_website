module.exports = {
    darkMode: 'media',
    content: ['./templates/**/*.{html.tera,js}'],
    theme: {
        extend: {
            spacing: {
                '128': '32rem',
                '144': '36rem',
                '160': '40rem',
                '176': '44rem',
                '192': '48rem',
                '208': '52rem',
                '224': '56rem',
                '240': '60rem',
                '256': '64rem',
                '272': '68rem',
                '288': '72rem',
                '304': '76rem',
                '320': '80rem',
                '336': '84rem',
                '352': '88rem',
                '368': '92rem',
                '384': '96rem',
                '400': '100rem',
                '416': '104rem',
                '432': '108rem',
                '448': '112rem',
                '464': '116rem',
                '480': '120rem',
                '496': '124rem',
                '512': '128rem',
                '528': '132rem',
                '544': '136rem',
                '560': '140rem',
                '576': '144rem',
                '592': '148rem',
                '608': '152rem'
            }
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
};