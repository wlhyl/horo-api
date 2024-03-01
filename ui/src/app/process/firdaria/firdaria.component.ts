import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ApiService } from 'src/app/services/api/api.service';
import { Horoconfig } from 'src/app/services/config/horo-config.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { FirdariaRequest } from 'src/app/type/interface/request-data';
import { FirdariaPeriod } from 'src/app/type/interface/respone-data';

@Component({
  selector: 'app-firdaria',
  templateUrl: './firdaria.component.html',
  styleUrls: ['./firdaria.component.scss'],
})
export class FirdariaComponent implements OnInit {
  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  title = '法达';

  horoData = this.storage.horoData;

  firdariaData: Array<FirdariaPeriod> = [];

  constructor(
    private api: ApiService,
    private storage: HorostorageService,
    public config: Horoconfig,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);

    const requestData: FirdariaRequest = {
      native_date: this.horoData.date,
      geo: this.horoData.geo,
    };

    this.api.firdaria(requestData).subscribe({
      next: (respone) => (this.firdariaData = respone),
      error: (error) => {
        const message = error.message + ' ' + error.error.message;
        this.message = message;
        this.isAlertOpen = true;
      },
    });
  }
}
