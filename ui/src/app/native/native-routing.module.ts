import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { NativePage } from './native.page';
import { ImageComponent } from './image.component';

const routes: Routes = [
  {
    path: '',
    component: NativePage,
  },
  { path: 'image', component: ImageComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class NativePageRoutingModule {}
