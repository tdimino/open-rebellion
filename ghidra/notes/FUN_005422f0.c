
uint __thiscall
FUN_005422f0(void *this,int *param_1,int param_2,int param_3,int *param_4,void *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  void *pvVar4;
  undefined3 extraout_var_00;
  uint uVar5;
  undefined4 uStack_4c;
  int aiStack_48 [2];
  int aiStack_40 [2];
  int aiStack_38 [2];
  undefined4 uStack_30;
  int iStack_2c;
  int aiStack_28 [2];
  uint auStack_20 [4];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647088;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_4 + 4))();
  uStack_30 = 1;
  iStack_2c = -1;
  uStack_4 = 0;
  FUN_00520580(param_5,&uStack_30);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(aiStack_38);
  uStack_4 = 1;
  FUN_004f4340(aiStack_40);
  uStack_4._0_1_ = 2;
  FUN_004f4340(aiStack_48);
  uStack_4._0_1_ = 3;
  FUN_004f4340(aiStack_28);
  uStack_4._0_1_ = 4;
  bVar1 = FUN_0054bb90(param_1,param_2,param_3,aiStack_38,aiStack_40,aiStack_48,aiStack_28,param_5);
  uVar5 = (uint)(CONCAT31(extraout_var,bVar1) != 0);
  FUN_004ece30(&uStack_4c);
  uStack_4._0_1_ = 5;
  if (*(int *)((int)param_5 + 4) == -1) {
    iVar2 = FUN_0054bf00((int)aiStack_28,&uStack_4c,param_5);
    if ((iVar2 == 0) || (uVar5 == 0)) {
      uVar5 = 0;
    }
    else {
      uVar5 = 1;
    }
    if (*(int *)((int)param_5 + 4) == -1) {
      iVar2 = FUN_0054c200(param_1,(int)aiStack_38,(int)aiStack_40,(int)aiStack_48,param_5);
      if ((iVar2 == 0) || (uVar5 == 0)) {
        uVar5 = 0;
      }
      else {
        uVar5 = 1;
      }
    }
  }
  FUN_00568fe0(auStack_20);
  uStack_4._0_1_ = 6;
  if (*(int *)((int)param_5 + 4) == -1) {
    iVar2 = FUN_005830a0((int)aiStack_38,(int)aiStack_40,(int)aiStack_48,auStack_20,param_5);
    if ((iVar2 == 0) || (uVar5 == 0)) {
      uVar5 = 0;
    }
    else {
      uVar5 = 1;
    }
  }
  if (*(int *)((int)param_5 + 4) == -1) {
    if (iStack_10 == 1) {
      iVar2 = 1;
    }
    else if (iStack_10 == 2) {
      iVar2 = 2;
    }
    else {
      iVar2 = 0;
    }
    piVar3 = FUN_004f2d10(iVar2,(uint *)(param_1 + 2));
    if (piVar3 == (int *)0x0) {
      uStack_30 = 0x40;
      iStack_2c = 0x21;
      uStack_4._0_1_ = 7;
      FUN_00520580(param_5,&uStack_30);
      uStack_4._0_1_ = 6;
      FUN_00619730();
    }
    if (*(int *)((int)param_5 + 4) == -1) {
      for (pvVar4 = (void *)FUN_0051cad0(0x40); pvVar4 != (void *)0x0;
          pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
        if (*(int *)((int)pvVar4 + 0x5c) == 0) {
          uStack_30 = 1;
          iStack_2c = -1;
          uStack_4 = CONCAT31(uStack_4._1_3_,8);
          iVar2 = FUN_0054c590(this,iStack_10,aiStack_38,aiStack_40,aiStack_48,auStack_20,piVar3,
                               pvVar4,&uStack_30);
          if ((iVar2 == 0) || (uVar5 == 0)) {
            uVar5 = 0;
          }
          else {
            uVar5 = 1;
          }
          if (iStack_2c == -1) {
            bVar1 = FUN_0054bac0(pvVar4,param_4);
            uVar5 = CONCAT31(extraout_var_00,bVar1);
          }
          uStack_4._0_1_ = 6;
          FUN_00619730();
        }
      }
    }
  }
  uStack_4._0_1_ = 5;
  FUN_00619730();
  uStack_4._0_1_ = 4;
  FUN_00619730();
  uStack_4._0_1_ = 3;
  FUN_004f4380(aiStack_28);
  uStack_4._0_1_ = 2;
  FUN_004f4380(aiStack_48);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_004f4380(aiStack_40);
  uStack_4 = 0xffffffff;
  FUN_004f4380(aiStack_38);
  ExceptionList = pvStack_c;
  return uVar5;
}

