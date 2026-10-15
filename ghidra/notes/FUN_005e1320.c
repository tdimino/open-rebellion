
undefined4 FUN_005e1320(int *param_1,uint param_2,uint param_3,undefined4 param_4)

{
  HWND hWnd;
  undefined4 uVar1;
  HDC pHVar2;
  undefined4 uVar3;
  tagPAINTSTRUCT tStack_40;
  
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      hWnd = (HWND)param_1[6];
      pHVar2 = BeginPaint(hWnd,&tStack_40);
      (**(code **)(*param_1 + 0x28))(pHVar2);
      FUN_005e22a0(pHVar2);
      EndPaint(hWnd,(PAINTSTRUCT *)&stack0xffffffbc);
      return 0;
    }
    if (param_2 == 1) {
      uVar3 = 0x51b;
      param_1[9] = 0x17;
      uVar1 = FUN_006037f0(6,0x51b);
      FUN_005ef1d0(uVar1,uVar3);
      FUN_005e2320();
      FUN_005e1fd0();
      (**(code **)(*param_1 + 0x48))();
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
  uVar1 = FUN_005ff530(param_1,param_2,param_3,param_4);
  return uVar1;
}

