
int __thiscall FUN_0054ac70(void *param_1,uint param_2,uint param_3)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar6;
  int aiStack_2c [3];
  int aiStack_20 [5];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647e90;
  pvStack_c = ExceptionList;
  iVar6 = 1;
  if (param_2 != 0) {
    aiStack_20[1] = 0;
    aiStack_20[0] = 0;
    ExceptionList = &pvStack_c;
    FUN_004ece30(&param_2);
    iStack_4 = 0;
    uVar3 = FUN_005434b0(aiStack_20 + 1,aiStack_20,&param_2);
    aiStack_2c[0] = 0;
    aiStack_20[2] = 0;
    FUN_004ece30(aiStack_2c + 2);
    iStack_4._0_1_ = 1;
    uVar4 = FUN_00543760(aiStack_2c,aiStack_20 + 2,aiStack_2c + 2);
    if ((uVar4 == 0) || (uVar3 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    if (aiStack_2c[0] != 0) {
      puVar5 = FUN_004ece40(&param_2);
      if (puVar5 != (uint *)0x0) {
        FUN_004f4340(aiStack_20 + 3);
        aiStack_2c[1] = 0x35000281;
        iStack_4._0_1_ = 3;
        bVar1 = FUN_004f44b0(aiStack_20 + 3,aiStack_2c + 1,0);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar6 == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        iStack_4._0_1_ = 2;
        FUN_00619730();
        bVar2 = FUN_00547340(param_1,*(uint *)(aiStack_2c[0] + 0x24) >> 6 & 3,&param_2,
                             (int)(aiStack_20 + 3),param_3);
        if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        iStack_4._0_1_ = 1;
        FUN_004f4380(aiStack_20 + 3);
      }
    }
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    iStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return iVar6;
}

