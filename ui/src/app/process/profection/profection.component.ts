import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { ProfectionRequest } from 'src/app/type/interface/request-data';
import { Profection } from 'src/app/type/interface/respone-data';

@Component({
  selector: 'app-profection',
  templateUrl: './profection.component.html',
  styleUrls: ['./profection.component.scss'],
})
export class ProfectionComponent implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;
  profection: Profection = {
    year_house: 0,
    month_house: 0,
    day_house: 0,
    date_per_house: [],
  };

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  title = '小限';

  constructor(
    private api: ApiService,
    private storage: HorostorageService,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);

    const profectionData: ProfectionRequest = {
      native_date: this.horoData.date,
      process_date: this.processData.date,
    };
    this.api.profection(profectionData).subscribe({
      next: (respone) => (this.profection = respone),
      error: (error) => {
        const message = error.message + ' ' + error.error.message;
        this.message = message;
        this.isAlertOpen = true;
      },
    });
  }
}
