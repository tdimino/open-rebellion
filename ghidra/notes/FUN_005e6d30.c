
undefined4 FUN_005e6d30(int *param_1,uint param_2,uint param_3,undefined4 param_4)

{
  HWND hWnd;
  HDC pHVar1;
  undefined4 uVar2;
  tagPAINTSTRUCT tStack_40;
  
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      hWnd = (HWND)param_1[6];
      pHVar1 = BeginPaint(hWnd,&tStack_40);
      (**(code **)(*param_1 + 0x28))(pHVar1);
      FUN_005e7ac0(pHVar1);
      if (param_1[0x2e] != 0) {
        FUN_00609a00();
      }
      EndPaint(hWnd,(PAINTSTRUCT *)&stack0xffffffbc);
      return 0;
    }
    if (param_2 == 1) {
      param_1[9] = 0x16;
      FUN_005e6f60();
      (**(code **)(*param_1 + 0x48))();
      FUN_005e7800();
      FUN_005e79b0();
      return 0;
    }
  }
  else {
    switch(param_2) {
    case 0x2b:
      (**(code **)(*param_1 + 0x50))(param_4);
      return 0;
    case 0x100:
    case 0x101:
      (**(code **)(*param_1 + 0x58))(param_2,param_3);
      return 0;
    case 0x111:
      (**(code **)(*param_1 + 0x54))(param_3 & 0xffff);
      return 0;
    }
  }
  uVar2 = FUN_005ff530(param_1,param_2,param_3,param_4);
  return uVar2;
}

