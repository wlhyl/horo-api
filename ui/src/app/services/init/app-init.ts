import * as FontFaceObserver from 'fontfaceobserver';
import { Horoconfig } from '../config/horo-config.service';
import { ApiService } from '../api/api.service';
import { lastValueFrom } from 'rxjs';

export async function appInit(config: Horoconfig, api: ApiService) {
  const font = config.astrologyFont;
  const customFont = new FontFaceObserver(font);
  try {
    await customFont.load();
  } catch (error) {
    const message = `字体 “${font}” 加载失败! '${error}'`;
    console.log(message);
    throw error;
  }

  try {
    config.houses = await lastValueFrom(api.getHouses());
  } catch (error) {
    const message = '从server获取宫位系统失败!';
    console.log(message);
    throw error;
  }
}
