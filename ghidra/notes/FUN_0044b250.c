
UINT FUN_0044b250(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  WPARAM wParam;
  UINT UVar1;
  int iVar2;
  
  if (param_2 == 1) {
    wParam = param_1[6];
    iVar2 = FUN_00422ca0();
    PostMessageA(*(HWND *)(iVar2 + 0x18),0x467,wParam,0);
    FUN_00606650(param_1,1,param_3,param_4);
    return 0;
  }
  if (param_2 != 0x40d) {
    UVar1 = FUN_00606650(param_1,param_2,param_3,param_4);
    return UVar1;
  }
  (**(code **)(*param_1 + 0x30))();
  return 0;
}

