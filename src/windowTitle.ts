import { appWindow } from '@tauri-apps/api/window'

export function initTitleBar() {
  document
    .getElementById('titlebar-minimize')
    .addEventListener('click', () => appWindow.minimize())
  
  // TODO: option to toggle and return to previous size
  document
    .getElementById('titlebar-maximize')
    .addEventListener('click', () => appWindow.toggleMaximize())

  document
    .getElementById('titlebar-close')
    .addEventListener('click', () => appWindow.close())
}
