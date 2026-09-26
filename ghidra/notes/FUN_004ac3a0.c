
UINT FUN_004ac3a0(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  UINT UVar1;
  
  if (param_2 < 0x101) {
    if (param_2 == 0x100) {
      if (param_3 == (HPALETTE)0x1b) {
        (**(code **)(*param_1 + 0x30))();
        return 0;
      }
      FUN_00606650(param_1,0x100,param_3,param_4);
      return 0;
    }
    if (param_2 == 0xf) {
      UVar1 = FUN_00606650(param_1,0xf,param_3,param_4);
      return UVar1;
    }
  }
  else if (param_2 < 0x205) {
    if ((param_2 == 0x204) || (param_2 == 0x201)) {
LAB_004ac471:
      if (param_1[0x4e] != 0) {
        (**(code **)(*param_1 + 0x80))();
      }
      UVar1 = FUN_00606650(param_1,param_2,param_3,param_4);
      return UVar1;
    }
  }
  else {
    if (param_2 == 0x407) {
      (**(code **)(*param_1 + 0x7c))();
      return 0;
    }
    if (param_2 == 0x40e) {
      PostMessageA(*(HWND *)(param_1[8] + 0x18),0x467,param_1[6],0);
      goto LAB_004ac471;
    }
  }
  UVar1 = FUN_00606650(param_1,param_2,param_3,param_4);
  return UVar1;
}

