
HGDIOBJ FUN_005def00(int *param_1,uint param_2,HWND param_3,LPARAM param_4)

{
  HWND hWnd;
  undefined4 uVar1;
  HDC pHVar2;
  HGDIOBJ pvVar3;
  uint uVar4;
  tagPAINTSTRUCT tStack_40;
  
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      hWnd = (HWND)param_1[6];
      pHVar2 = BeginPaint(hWnd,&tStack_40);
      (**(code **)(*param_1 + 0x28))(pHVar2);
      EndPaint(hWnd,(PAINTSTRUCT *)&stack0xffffffbc);
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 1) {
      uVar4 = 0x51c;
      param_1[9] = 0x19;
      uVar1 = FUN_006037f0(6);
      FUN_005ef1d0(param_1,uVar1,uVar4);
      (**(code **)(*param_1 + 0x48))();
      FUN_005df6b0((int)param_1);
      return (HGDIOBJ)0x0;
    }
  }
  else {
    switch(param_2) {
    case 0x2b:
      (**(code **)(*param_1 + 0x50))(param_4);
      return (HGDIOBJ)0x0;
    case 0x100:
    case 0x101:
      (**(code **)(*param_1 + 0x58))(param_2,param_3);
      return (HGDIOBJ)0x0;
    case 0x111:
      (**(code **)(*param_1 + 0x54))((uint)param_3 & 0xffff);
      return (HGDIOBJ)0x0;
    }
  }
  pvVar3 = FUN_005ff530(param_1,param_2,param_3,param_4);
  return pvVar3;
}

