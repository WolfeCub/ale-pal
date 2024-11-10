import { mount } from 'svelte'
import App from './App.svelte'
import { initializeRouting } from './routing.svelte'

initializeRouting();

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
