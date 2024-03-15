import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ActivatedRoute, Router } from '@angular/router';
import { Path } from './path';

@Component({
  selector: 'app-qizheng',
  templateUrl: './qizheng.page.html',
  styleUrls: ['./qizheng.page.scss'],
})
export class QizhengPage implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;

  title = '七政四余';

  constructor(
    private storage: HorostorageService,
    private titleService: Title,
    private router: Router,
    private route: ActivatedRoute
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }

  getProcess() {
    this.storage.horoData = this.horoData;
    this.storage.processData = this.processData;
    const path = Path.Horo;
    this.router.navigate([path], {
      relativeTo: this.route,
    });
  }
}
