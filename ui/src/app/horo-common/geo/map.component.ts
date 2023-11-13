import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';

declare let BMap :any
declare let BMAP_STATUS_SUCCESS :any

@Component({
  selector: 'horo-map',
  templateUrl: './map.component.html',
  styleUrls: ['./map.component.scss'],
})
export class MapComponent implements OnInit {
  isModalOpen = false

  @Input()
  localName :string = ""
  @Input()
  long :number = 116
  @Input()
  lat :number = 28

  @Output()
  localNameChange = new EventEmitter<string>()
  @Output()
  longChange = new EventEmitter<number>()
  @Output()
  latChange = new EventEmitter<number>()

  query = {
    // geo_name: "",
    // geo_long: 360, //360，表明没有查询到值
    // geo_lat: 360,
    errorMessage: '',
    error: false,
  }

  private map :any

  constructor() { }

  ngOnInit() {}

  ok():void{
    this.localNameChange.emit(this.localName)
    this.longChange.emit(this.long)
    this.latChange.emit(this.lat)
    this.isModalOpen = false
  }
  cancel() :void{
    this.isModalOpen = false
  }
  open():void{
    this.isModalOpen = true
  }
   shown():void{
    this.query = {
      errorMessage: '',
      error: false,
    }
    // 创建地图实例
    this.map = new BMap.Map("container", {
      coordsType: 5 // coordsType指定输入输出的坐标类型，3为gcj02坐标，5为bd0ll坐标，默认为5。
                   // 指定完成后API将以指定的坐标类型处理您传入的坐标
    })

    // 创建点坐标
    const point = new BMap.Point(this.long, this.lat)
    this.map.centerAndZoom(point, 15)
    this.map.enableScrollWheelZoom(true);     //开启鼠标滚轮缩放
    this.map.enableContinuousZoom()
    this.map.addControl(new BMap.NavigationControl())
    this.map.addControl(new BMap.OverviewMapControl())

    let geoc = new BMap.Geocoder()
    this.map.addEventListener("click", (e :any) =>{
      let pt = e.point
      this.query.error = false
      this.long = pt.lng
      this.lat = pt.lat
      // 添加标
      geoc.getLocation(pt, (rs :any) => {
        // this.map.centerAndZoom(pt, 15)
        this.map.clearOverlays()
        let marker = new BMap.Marker(pt) // 创建标注
        this.map.addOverlay(marker)
        marker.openInfoWindow(new BMap.InfoWindow(rs.address))
        this.localName = rs.address
      })
    })

  }

  queryGeo(){
    let local_search :any
    let options = {
        renderOptions: {map: this.map}, 
        onSearchComplete: (results :any) => {
        if (local_search.getStatus() != BMAP_STATUS_SUCCESS){
          this.query.error = true
          this.query.errorMessage = "查询错误"
          return;
        } 
        if(results.getCurrentNumPois() != 1){
          this.query.error = true;
          this.query.errorMessage = "请缩小查询范围";
          return;
        }
        this.query.error = false;
        this.long = results.getPoi(0).point.lng;
        this.lat = results.getPoi(0).point.lat;
        }
      }
      local_search = new BMap.LocalSearch(this.map, options);      
      local_search.search(this.localName); 
      }

}
