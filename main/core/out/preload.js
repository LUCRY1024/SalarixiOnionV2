import { contextBridge, ipcRenderer } from 'electron';
contextBridge.exposeInMainWorld('client', {
    port: () => ipcRenderer.invoke('client', 'port').then(answer => answer.result),
    getInfo: () => ipcRenderer.invoke('client', 'get-info').then(answer => answer.result),
    window: (action) => ipcRenderer.invoke('client', 'window', { action: action }).then(answer => answer.result),
    openFile: () => ipcRenderer.invoke('client', 'open-file').then(answer => answer.result),
    openUrl: (url) => ipcRenderer.invoke('client', 'open-url', { url: url }).then(answer => answer.result),
    loadConfig: () => ipcRenderer.invoke('client', 'load-config').then(answer => answer.result),
    saveConfig: (config) => ipcRenderer.invoke('client', 'save-config', { config: config }).then(answer => answer.result),
    scrapeProxies: (options) => ipcRenderer.invoke('client', 'scrape-proxies', options).then(answer => answer.result)
});
