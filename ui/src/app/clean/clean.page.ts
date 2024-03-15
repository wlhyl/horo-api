import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { HorostorageService } from '../services/horostorage/horostorage.service';

@Component({
  selector: 'app-clean',
  templateUrl: './clean.page.html',
  styleUrls: ['./clean.page.scss'],
})
export class CleanPage implements OnInit {
  title = '清除缓存';
  message = '';

  constructor(private titleService: Title) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }

  clean() {
    this.message = '正在清除缓存...';
    localStorage.removeItem('horo_data');
    localStorage.removeItem('process_data');
    this.message = '清除缓存完成';
  }
}
