import { NgModule } from '@angular/core';
import { PreloadAllModules, RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: 'home',
    loadChildren: () => import('./home/home.module').then( m => m.HomePageModule)
  },
  {
    path: '',
    redirectTo: 'home',
    pathMatch: 'full'
  },
  {
    path: 'native',
    loadChildren: () => import('./native/native.module').then( m => m.NativePageModule)
  },
  {
    path: 'process',
    loadChildren: () => import('./process/process.module').then( m => m.ProcessPageModule)
  },
  {
    path: 'qizheng',
    loadChildren: () => import('./qizheng/qizheng.module').then( m => m.QizhengPageModule)
  },
  {
    path: 'clean',
    loadChildren: () => import('./clean/clean.module').then( m => m.CleanPageModule)
  },
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, { preloadingStrategy: PreloadAllModules })
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
