
uint __thiscall FUN_00571410(int param_1,undefined4 param_2,int param_3,undefined4 param_4)

{
  bool bVar1;
  int iVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined1 auStack_44 [52];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  iVar5 = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064c250;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_00592f50(param_2,param_3,param_4);
  uVar4 = (uint)(iVar2 != 0);
  if (iVar5 == 2) {
    FUN_004f4340();
    uStack_4 = 0;
    FUN_00526090(param_1);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_00525930();
    while (iStack_10 != 0) {
      puVar6 = &param_2;
      FUN_00525f20(puVar6);
      uVar3 = FUN_004025b0(puVar6);
      uStack_4._0_1_ = 2;
      uVar4 = FUN_004f44b0(uVar3,0);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_00619730();
      FUN_005258f0();
    }
    param_3 = 0;
    iVar5 = FUN_00570e40(*(uint *)(param_1 + 0x24) >> 6 & 3,auStack_44,&param_3);
    if ((iVar5 == 0) || (uVar4 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if ((param_3 == 0) || (!bVar1)) {
      uVar4 = 0;
    }
    else {
      uVar4 = 1;
    }
    if (param_3 != 0) {
      uVar3 = FUN_004025b0(&param_2);
      uStack_4._0_1_ = 3;
      iVar5 = FUN_005703b0(uVar3,param_4);
      if ((iVar5 == 0) || (uVar4 == 0)) {
        uVar4 = 0;
      }
      else {
        uVar4 = 1;
      }
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_00619730();
    }
    uStack_4 = uStack_4 & 0xffffff00;
    FUN_00526130();
    uStack_4 = 0xffffffff;
    FUN_004f4380();
  }
  ExceptionList = pvStack_c;
  return uVar4;
}

