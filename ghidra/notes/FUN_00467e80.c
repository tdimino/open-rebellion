
UINT FUN_00467e80(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  UINT UVar1;
  
  if (param_2 == 7) {
    SetFocus(*(HWND *)(param_1[0x5a] + 0x18));
  }
  else {
    if (param_2 == 0x40d) {
      (**(code **)(*param_1 + 0x30))();
      return 0;
    }
    if (param_2 != 0x483) {
      UVar1 = FUN_00606650(param_1,param_2,param_3,param_4);
      return UVar1;
    }
    if ((param_3 == (HPALETTE)0xbbe) && (param_1[0x46] != 1)) {
      (**(code **)(*param_1 + 0x48))(0x65,0,0);
      return 0;
    }
  }
  return 0;
}

