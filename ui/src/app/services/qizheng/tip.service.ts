import { Injectable } from '@angular/core';
import { fabric } from 'fabric';
import { QizhengConfigService } from '../config/qizheng-config.service';

@Injectable({
  providedIn: 'root',
})
export class TipService {
  private object: fabric.Object | null = null;

  constructor(private config: QizhengConfigService) {}

  newTip(
    message: string,
    object: fabric.Object,
    canvas: fabric.Canvas
  ): fabric.Object {
    const noteText = new fabric.Text(message, {
      fontSize: this.config.fontSize,
      // fontFamily: config.textFont,
      selectable: false,
      backgroundColor: this.config.noteTextColor,
    });

    noteText.on('mousedown', (e) => {
      canvas.remove(noteText);
      this.object = null;
    });

    // 计算tip的left,top
    let top = object.top! + object.height! / 2 - noteText.height! / 2;
    let left = object.left! + object.width!;

    const right = left + noteText.width!;
    if (right > canvas.width!) {
      left = object.left! - noteText.width!;
    }

    if (top < 0) {
      top = object.top!;
    }

    const botton = top + noteText.height!;
    if (botton > canvas.height!) {
      top = object.top! + object.height! - noteText.height!;
    }

    noteText.top = top;
    noteText.left = left;

    // if (this.object) canvas.remove(this.object);

    // this.object = noteText;

    return noteText;
  }

  showTip(object: fabric.Object, canvas: fabric.Canvas) {
    if (this.object) canvas.remove(this.object);
    this.object = object;
    canvas.add(object);
  }
}
