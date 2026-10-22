
HGDIOBJ FUN_0059ab40(int *param_1,uint param_2,HWND param_3,uint param_4)

{
  HWND hWnd;
  void *pvVar1;
  HDC pHVar2;
  int *piVar3;
  HGDIOBJ pvVar4;
  int iVar5;
  tagPAINTSTRUCT tStack_4c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006518db;
  pvStack_c = ExceptionList;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      hWnd = (HWND)param_1[6];
      ExceptionList = &pvStack_c;
      pHVar2 = BeginPaint(hWnd,&tStack_4c);
      (**(code **)(*param_1 + 0x28))(pHVar2);
      EndPaint(hWnd,&tStack_4c);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 1) {
      ExceptionList = &pvStack_c;
      FUN_00610be0(*(undefined4 *)(param_1[8] + 0x18),0x11,0);
      param_1[9] = 0xb;
      pvVar1 = (void *)FUN_00618b70(0x690);
      uStack_4 = 0;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_005c14d0(pvVar1,param_1[7],0x11,0x1e,0x1b7,0x1b5,param_1,0,1,param_1[0x27],0);
      }
      uStack_4 = 0xffffffff;
      param_1[0x42] = (int)pvVar1;
      FUN_0059ae70(param_1);
      FUN_0059b080(param_1);
      FUN_0059cac0((int)param_1);
      FUN_005ffce0((void *)param_1[0x42],0);
      FUN_005c1c10((int *)param_1[0x42]);
      (**(code **)(*(int *)param_1[0x42] + 4))(5);
      UpdateWindow((HWND)param_1[6]);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
  }
  else if (param_2 < 0x19) {
    if (param_2 == 0x18) {
      if (param_3 == (HWND)0x0) {
        return (HGDIOBJ)0x0;
      }
      ExceptionList = &pvStack_c;
      if (param_1[0x42] != 0) {
        ExceptionList = &pvStack_c;
        FUN_005c3080(param_1[0x42]);
        FUN_005c3020(param_1[0x42]);
      }
      FUN_0059afb0((int)param_1);
      pvVar4 = (HGDIOBJ)FUN_006007b0(param_1,0x18,(uint)param_3,param_4);
      ExceptionList = pvStack_c;
      return pvVar4;
    }
    if (param_2 == 0x10) {
      ExceptionList = &pvStack_c;
      FUN_00610c00(0);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
  }
  else if (param_2 < 0x102) {
    if (0xff < param_2) {
      ExceptionList = &pvStack_c;
      FUN_0059b730(param_1,param_2,(uint)param_3,param_4);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x2b) {
      ExceptionList = &pvStack_c;
      FUN_0059b100(param_1,param_4);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
  }
  else if (param_2 < 0x112) {
    if (param_2 == 0x111) {
      ExceptionList = &pvStack_c;
      FUN_0059b130(param_1,(uint)param_3 & 0xffff);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x102) {
      ExceptionList = &pvStack_c;
      FUN_0059b730(param_1,0x102,(uint)param_3,0);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
  }
  else {
    if (0x204 < param_2) {
      if (param_2 < 0x403) {
        if (param_2 != 0x402) {
          if (param_2 != 0x30f) {
            if (param_2 != 0x311) goto LAB_0059adeb;
            if (param_3 == (HWND)param_1[6]) {
              return (HGDIOBJ)0x0;
            }
          }
          ExceptionList = &pvStack_c;
          FUN_006001f0((int)param_1);
          piVar3 = (int *)param_1[0x42];
          if (piVar3 == (int *)0x0) {
            ExceptionList = pvStack_c;
            return (HGDIOBJ)0x0;
          }
          pvVar4 = (HGDIOBJ)(**(code **)(*piVar3 + 0x14))(piVar3,param_2,param_3,param_4);
          ExceptionList = pvStack_c;
          return pvVar4;
        }
        ExceptionList = &pvStack_c;
        FUN_00610be0(*(undefined4 *)(param_1[8] + 0x18),0x11,0);
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 4))();
        FUN_0059afb0((int)param_1);
        iVar5 = 0;
      }
      else if (param_2 == 0x479) {
        iVar5 = 1;
        ExceptionList = &pvStack_c;
      }
      else {
        if (param_2 != 0x47a) goto LAB_0059adeb;
        ExceptionList = &pvStack_c;
        FUN_0059afb0((int)param_1);
        iVar5 = 0;
      }
      FUN_0059cd60(param_1,iVar5);
      ExceptionList = pvStack_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x204) {
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x201) {
      return (HGDIOBJ)0x0;
    }
  }
LAB_0059adeb:
  ExceptionList = &pvStack_c;
  pvVar4 = FUN_005ff530(param_1,param_2,param_3,param_4);
  ExceptionList = pvStack_c;
  return pvVar4;
}

