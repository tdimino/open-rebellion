
int __thiscall FUN_0054adf0(void *param_1,int param_2,uint param_3)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar6;
  uint uStack_38;
  int aiStack_34 [3];
  undefined4 uStack_28;
  int aiStack_24 [6];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647ed0;
  pvStack_c = ExceptionList;
  if (param_2 != 0) {
    aiStack_24[1] = 0;
    aiStack_24[0] = 0;
    ExceptionList = &pvStack_c;
    FUN_004ece30(&uStack_38);
    iStack_4 = 0;
    uVar3 = FUN_00543800(aiStack_24 + 1,aiStack_24,&uStack_38);
    param_2 = 0;
    aiStack_24[2] = 0;
    FUN_004ece30(&uStack_28);
    iStack_4._0_1_ = 1;
    uVar4 = FUN_00543760(&param_2,aiStack_24 + 2,&uStack_28);
    if ((uVar4 == 0) || (uVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    aiStack_34[0] = 0;
    aiStack_24[3] = 0;
    FUN_004ece30(aiStack_34 + 2);
    iStack_4._0_1_ = 2;
    uVar3 = FUN_005434b0(aiStack_34,aiStack_24 + 3,aiStack_34 + 2);
    if ((uVar3 == 0) || (!bVar1)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    if ((((param_2 != 0) && (aiStack_34[0] != 0)) &&
        (puVar5 = FUN_004ece40(&uStack_38), puVar5 != (uint *)0x0)) && (iVar6 != 0)) {
      if (((((byte)*(undefined4 *)(param_2 + 0x24) & 0xc0) == 0x80) &&
          (((byte)*(undefined4 *)(aiStack_34[0] + 0x24) & 0xc0) == 0x80)) &&
         (((*(byte *)(param_2 + 0x78) & 0x80) == 0 &&
          ((*(byte *)(aiStack_34[0] + 0x78) & 0x80) == 0)))) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      iVar6 = 0;
      if (bVar1) {
        FUN_004f4340(aiStack_24 + 4);
        aiStack_34[1] = 0x35000281;
        iStack_4._0_1_ = 4;
        bVar1 = FUN_004f44b0(aiStack_24 + 4,aiStack_34 + 1,0);
        iStack_4._0_1_ = 3;
        FUN_00619730();
        aiStack_34[1] = 0x32000242;
        iStack_4._0_1_ = 5;
        bVar2 = FUN_004f44b0(aiStack_24 + 4,aiStack_34 + 1,0);
        if ((CONCAT31(extraout_var_00,bVar2) == 0) || (CONCAT31(extraout_var,bVar1) == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        iStack_4._0_1_ = 3;
        FUN_00619730();
        bVar2 = FUN_00547340(param_1,*(uint *)(param_2 + 0x24) >> 6 & 3,&uStack_38,
                             (int)(aiStack_24 + 4),param_3);
        if ((CONCAT31(extraout_var_01,bVar2) == 0) || (!bVar1)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        iStack_4._0_1_ = 2;
        FUN_004f4380(aiStack_24 + 4);
      }
    }
    iStack_4._0_1_ = 1;
    FUN_00619730();
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00619730();
    iStack_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = pvStack_c;
    return iVar6;
  }
  return 1;
}

