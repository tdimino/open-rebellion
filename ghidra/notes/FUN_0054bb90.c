
bool FUN_0054bb90(int *param_1,int param_2,int param_3,int *param_4,int *param_5,int *param_6,
                 int *param_7,void *param_8)

{
  void *pvVar1;
  int *piVar2;
  bool bVar3;
  short sVar4;
  int iVar5;
  void *pvVar6;
  uint *puVar7;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *pvVar8;
  undefined4 *puVar9;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  int *piVar10;
  bool bVar11;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  piVar10 = param_4;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647fb8;
  pvStack_c = ExceptionList;
  bVar11 = true;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_4 + 4))();
  (**(code **)(*param_5 + 4))();
  (**(code **)(*param_6 + 4))();
  (**(code **)(*param_7 + 4))();
  pvVar8 = param_8;
  uStack_14 = 1;
  uStack_10 = 0xffffffff;
  uStack_4 = 0;
  FUN_00520580(param_8,&uStack_14);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  piVar2 = param_1;
  if (*(int *)((int)pvVar8 + 4) == -1) {
    iVar5 = FUN_0053f150(param_1,param_2,piVar10,pvVar8);
    bVar11 = iVar5 != 0;
  }
  if (bVar11) {
    if (*(int *)((int)pvVar8 + 4) == -1) {
      iVar5 = FUN_0053f150(piVar2,param_3,param_5,pvVar8);
      bVar11 = iVar5 != 0;
    }
    if (bVar11) {
      if (*(int *)((int)pvVar8 + 4) != -1) {
        ExceptionList = pvStack_c;
        return true;
      }
      pvVar6 = (void *)thunk_FUN_005f5060((int)piVar10);
      bVar3 = true;
      while (pvVar1 = pvVar6, pvVar1 != (void *)0x0) {
        pvVar6 = *(void **)((int)pvVar1 + 0x10);
        param_1 = (int *)0x0;
        puVar7 = FUN_00403040(pvVar1,&param_2);
        uStack_4 = 1;
        bVar11 = FUN_00582c30(puVar7,&param_1);
        if ((CONCAT31(extraout_var,bVar11) == 0) || (!bVar3)) {
          bVar11 = false;
        }
        else {
          bVar11 = true;
        }
        uStack_4 = 0xffffffff;
        FUN_00619730();
        pvVar8 = param_8;
        bVar3 = bVar11;
        if ((param_1 != (int *)0x0) &&
           (iVar5 = (**(code **)(*param_1 + 0x1d4))(), pvVar8 = param_8, iVar5 != 0)) {
          FUN_005f4fa0(piVar10,(int)pvVar1);
          FUN_004f57b0(param_6,pvVar1);
          pvVar8 = param_8;
        }
      }
      pvVar6 = (void *)thunk_FUN_005f5060((int)param_5);
      while ((pvVar1 = pvVar6, pvVar1 != (void *)0x0 &&
             (pvVar8 = param_8, piVar10 = param_4, bVar11))) {
        pvVar6 = *(void **)((int)pvVar1 + 0x10);
        param_1 = (int *)0x0;
        puVar7 = FUN_00403040(pvVar1,&param_2);
        uStack_4 = 2;
        bVar11 = FUN_00582c30(puVar7,&param_1);
        bVar11 = CONCAT31(extraout_var_00,bVar11) != 0;
        uStack_4 = 0xffffffff;
        FUN_00619730();
        piVar10 = param_4;
        pvVar8 = param_8;
        if ((param_1 != (int *)0x0) &&
           (iVar5 = (**(code **)(*param_1 + 0x1d4))(), piVar10 = param_4, pvVar8 = param_8,
           iVar5 != 0)) {
          FUN_005f4fa0(param_5,(int)pvVar1);
          FUN_004f57b0(param_6,pvVar1);
          piVar10 = param_4;
          pvVar8 = param_8;
        }
      }
      sVar4 = FUN_005f50e0((int)piVar10);
      if (sVar4 == 0) {
        uStack_14 = 0x40;
        uStack_10 = 0x91;
        uStack_4 = 3;
        FUN_00520580(pvVar8,&uStack_14);
        uStack_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  if (*(int *)((int)pvVar8 + 4) == -1) {
    pvVar8 = (void *)thunk_FUN_005f5060((int)piVar10);
    piVar10 = param_7;
    for (; pvVar8 != (void *)0x0; pvVar8 = *(void **)((int)pvVar8 + 0x10)) {
      puVar9 = FUN_00403040(pvVar8,&param_8);
      piVar10 = param_7;
      uStack_4 = 4;
      bVar3 = FUN_004f44b0(param_7,puVar9,0);
      if ((CONCAT31(extraout_var_01,bVar3) == 0) || (!bVar11)) {
        bVar11 = false;
      }
      else {
        bVar11 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
    for (pvVar8 = (void *)thunk_FUN_005f5060((int)param_5); pvVar8 != (void *)0x0;
        pvVar8 = *(void **)((int)pvVar8 + 0x10)) {
      puVar9 = FUN_00403040(pvVar8,&param_5);
      uStack_4 = 5;
      bVar3 = FUN_004f44b0(piVar10,puVar9,0);
      if ((CONCAT31(extraout_var_02,bVar3) == 0) || (!bVar11)) {
        bVar11 = false;
      }
      else {
        bVar11 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
    for (pvVar8 = (void *)thunk_FUN_005f5060((int)param_6); pvVar8 != (void *)0x0;
        pvVar8 = *(void **)((int)pvVar8 + 0x10)) {
      puVar9 = FUN_00403040(pvVar8,&param_5);
      uStack_4 = 6;
      bVar3 = FUN_004f44b0(piVar10,puVar9,0);
      if ((CONCAT31(extraout_var_03,bVar3) == 0) || (!bVar11)) {
        bVar11 = false;
      }
      else {
        bVar11 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = pvStack_c;
  return bVar11;
}

