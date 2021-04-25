import m from 'mithril'

import home from './routes/home'
import play from './routes/play'

const root = document.body

m.route(root, '/home',
{
    '/home': home,
    '/play': play,
})