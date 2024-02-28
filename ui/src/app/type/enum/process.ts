export enum ProcessName {
  Profection = 'Profection',
  Transit = 'Transit',
  SolarReturn = 'SolarReturn',
  LunarReturn = 'LunarReturn',
  SolarcomparNative = 'SolarcomparNative',
  NativecomparSolar = 'NativecomparSolar',
  LunarcomparNative = 'LunarcomparNative',
  NativecomparLunar = 'NativecomparLunar',
}

export namespace ProcessName {
  export function name(process: ProcessName): string {
    switch (process) {
      case ProcessName.Profection:
        return '小限';
      case ProcessName.Transit:
        return '行运';
      case ProcessName.SolarReturn:
        return '日返';
      case ProcessName.LunarReturn:
        return '月返';
      case ProcessName.SolarcomparNative:
        return '日返比本命';
      case ProcessName.NativecomparSolar:
        return '本命比日返';
      case ProcessName.LunarcomparNative:
        return '月返比本命';
      case ProcessName.NativecomparLunar:
        return '本命比月返';
    }
  }

  export function path(process: ProcessName): string {
    switch (process) {
      case ProcessName.Profection:
        return 'profection';
      case ProcessName.Transit:
        return 'transit';
      case ProcessName.SolarReturn:
        return 'return/solar';
      case ProcessName.LunarReturn:
        return 'return/lunar';
      case ProcessName.SolarcomparNative:
        return 'return/solar_native';
      case ProcessName.NativecomparSolar:
        return 'return/native_solar';
      case ProcessName.LunarcomparNative:
        return 'return/lunar_native';
      case ProcessName.NativecomparLunar:
        return 'return/native_lunar';
    }
  }
}
