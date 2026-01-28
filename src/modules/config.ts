import { log } from '../logger';


interface ConfigElement {
  tag: string;
  id: string;
  value: string | number | boolean | null;
}

export function initConfig(): void {
  !localStorage.getItem('salarixionion:config') ? localStorage.setItem('salarixionion:config', JSON.stringify({}, null, 2)) : null;
  
  let latest: any = null;

  setInterval(async () => {
    const elements = document.querySelectorAll<HTMLElement>('[keep="true"]');
    const config: Record<string, ConfigElement> = {};

    for (const element of elements) {
      const tag = element.tagName.toLocaleLowerCase();
      if (tag === 'input') {
        const el = element as HTMLInputElement;
        if (el.type === 'checkbox') {
          config[el.id] = {
            tag: 'checkbox',
            id: el.id,
            value: el.checked
          };
        } else {
          config[el.id] = {
            tag: 'input',
            id: el.id,
            value: el.type === 'number' ? parseInt(el.value) : el.value
          };
        }
      }
    }

    if (JSON.stringify(config) === JSON.stringify(latest)) return;

    for (let attempts = 0; attempts < 4; attempts++) {
      localStorage.setItem('salarixionion:config', JSON.stringify(config, null, 2))
      break;
    }

    latest = config;
  }, 1500);
}

export function loadConfig(): void {
  log('Загрузка конфига...', 'system');

  try {
    let config = JSON.parse(localStorage.getItem('salarixionion:config') ?? '');

    if (!config) {
      log('Ошибка загрузки конфига: Config not found', 'error');
      return;
    }

    for (const [id, element] of Object.entries<ConfigElement>(config)) {
      const doc = document.getElementById(id);

      if (doc) {
        switch (element.tag) {
          case 'checkbox':
            (doc as HTMLInputElement).checked = Boolean(element.value);
            break;
          case 'input':
            typeof element.value === 'number' ? (doc as HTMLInputElement).valueAsNumber = element.value ? element.value : 0 : (doc as HTMLInputElement).value = element.value ? element.value.toString() : '';
            break;
        }
      }
    }

    log('Конфиг успешно загружен', 'system');
  } catch (error) {
    log(`Ошибка загрузки конфига: ${error}`, 'error');
  }
}