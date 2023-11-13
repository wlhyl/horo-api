import * as FontFaceObserver from 'fontfaceobserver';
import { Horoconfig } from '../config/horo-config.service';

export async function appInit(config: Horoconfig) {
  const font = config.astrologyFont;
  const customFont = new FontFaceObserver(font);
  try {
    await customFont.load();
  } catch (error) {
    const message = `字体 “${font}” 加载失败! '${error}'`;
    console.log(message);
    throw error;
  }
}
