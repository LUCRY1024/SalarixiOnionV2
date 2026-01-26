import { log } from '../logger';


export type Language = 'ru' | 'en';

let cache_map: any = null;

async function uploadTranslation(): Promise<any> {
  try {
    const response = await fetch('https://raw.githubusercontent.com/nullclyze/SalarixiOnion/refs/heads/main/salarixi.lang.json');

    if (!response.ok) {
      log('Ошибка загрузки перевода: Failed to send request', 'error');
      return;
    }

    const data = await response.json();

    return data['map'];
  } catch (error) {
    log(`Ошибка загрузки перевода: ${error}`, 'error');
  }
}

export async function translate(lang: Language) {
  try {
    let map: any = null;

    if (!cache_map) {
      map = await uploadTranslation();
    } else {
      map = cache_map;
    }

    if (map) {
      document.querySelectorAll<HTMLElement>('[translator-tag]').forEach(e => {
        const tag = e.getAttribute('translator-tag');

        if (tag) {
          for (const el of map) {
            if (el.tags.includes(tag)) {
              e.innerText = el.lang[lang];
            }
          }
        }
      });
    }
  } catch (error) {
    log(`Ошибка перевода: ${error}`, 'error');
  }
}