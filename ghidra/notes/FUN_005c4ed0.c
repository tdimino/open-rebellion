
HGDIOBJ FUN_005c4ed0(int *param_1,uint param_2,HWND param_3,int param_4)

{
  HWND hWnd;
  bool bVar1;
  undefined4 uVar2;
  void *pvVar3;
  uint *puVar4;
  HDC pHVar5;
  HGDIOBJ pvVar6;
  undefined3 extraout_var;
  int iVar7;
  int iVar8;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653b56;
  local_c = ExceptionList;
  if (param_2 < 0x10) {
    if (param_2 == 0xf) {
      hWnd = (HWND)param_1[6];
      ExceptionList = &local_c;
      pHVar5 = BeginPaint(hWnd,&local_4c);
      (**(code **)(*param_1 + 0x28))(pHVar5);
      EndPaint(hWnd,&local_4c);
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 1) {
      ExceptionList = &local_c;
      param_1[9] = 0x1d;
      uVar2 = FUN_006037f0(6);
      FUN_005c5110(param_1,uVar2,0x400);
      pvVar3 = (void *)FUN_00618b70(0xb8);
      local_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_0060e150(pvVar3,0x22,9,0x5f,6,(int)param_1,0x200ff00,0x2ff0000,0x54000000);
      }
      local_4 = 0xffffffff;
      param_1[0x28] = (int)pvVar3;
      if (pvVar3 != (void *)0x0) {
        *(undefined4 *)((int)pvVar3 + 0x24) = 0xde01;
        FUN_0060e400((void *)param_1[0x28],100,0);
        FUN_005ffce0((void *)param_1[0x28],0);
        (**(code **)(*(int *)param_1[0x28] + 4))(5);
      }
      pvVar3 = (void *)FUN_00618b70(0x114);
      local_4 = 1;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        iVar8 = 1;
        iVar7 = 0;
        puVar4 = (uint *)FUN_005ff440((int)param_1);
        pvVar3 = FUN_00602150(pvVar3,uVar2,0x89,2,0x15,0x14,param_1,0xde02,0x3fd,0x3fe,puVar4,iVar7,
                              iVar8);
      }
      local_4 = 0xffffffff;
      param_1[0x27] = (int)pvVar3;
      if (pvVar3 == (void *)0x0) {
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      FUN_006030c0(pvVar3,8);
      FUN_00603150((void *)param_1[0x27],2,0x3ff);
      FUN_00603150((void *)param_1[0x27],4,0x3fe);
      FUN_005ffce0((void *)param_1[0x27],0);
      (**(code **)(*(int *)param_1[0x27] + 0x10))();
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
  }
  else {
    if (param_2 == 0x2b) {
      ExceptionList = &local_c;
      FUN_005c5180(param_1,param_4);
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x111) {
      if ((short)param_3 != -0x21fe) {
        return (HGDIOBJ)0x0;
      }
      ExceptionList = &local_c;
      if (((void *)param_1[0x27] != (void *)0x0) &&
         (ExceptionList = &local_c, bVar1 = FUN_00603120((void *)param_1[0x27],4),
         CONCAT31(extraout_var,bVar1) == 1)) {
        FUN_005c0f60(1);
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      FUN_005c0f60(0);
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
  }
  ExceptionList = &local_c;
  pvVar6 = FUN_005ff530(param_1,param_2,param_3,param_4);
  ExceptionList = local_c;
  return pvVar6;
}

