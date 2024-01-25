import { Component, OnInit } from '@angular/core';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { ProfectionData } from 'src/app/type/interface/request-data';
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
  message=""

  constructor(private api: ApiService, private storage: HorostorageService) {}

  ngOnInit() {
    const profectionData: ProfectionData = {
      year: this.horoData.year,
      month: this.horoData.month,
      day: this.horoData.day,
      hour: this.horoData.hour,
      minute: this.horoData.minute,
      second: this.horoData.second,
      tz: this.horoData.tz,
      st: this.horoData.st,
      process_year: this.processData.year,
      process_month: this.processData.month,
      process_day: this.processData.day,
      process_hour: this.processData.hour,
      process_minute: this.processData.minute,
      process_second: this.processData.second,
    };
    this.api.profection(profectionData).subscribe({
      next: (respone) => (this.profection = respone),
      error: (error) => {
        const message = error.message + ' ' + error.error.message;
        this.message=message
        this.isAlertOpen=true
      },
    });
  }


}
