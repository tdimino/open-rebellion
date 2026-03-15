
undefined4 __thiscall
FUN_005fe140(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            int param_7)

{
  byte bVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  byte *pbVar12;
  byte *pbVar13;
  int local_30;
  int local_2c;
  int local_28;
  int local_24;
  
  iVar3 = FUN_005fc0e0((int *)this);
  bVar2 = FUN_005fc040((int *)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    if (*(int *)((int)this + 0x24) == 0) {
      return 0;
    }
    bVar1 = *(byte *)((int)this + 8);
    iVar4 = FUN_005fc110((int *)param_1);
    if (iVar4 != 0) {
      return 0;
    }
    iVar4 = FUN_005fc110((int *)this);
    if (iVar4 == 1) {
      uVar5 = FUN_005fe8d0();
      return uVar5;
    }
    iVar4 = FUN_00583c40((int)param_1);
    iVar6 = FUN_00583c40((int)this);
    local_30 = FUN_005fc0f0((int *)param_1);
    iVar7 = FUN_005fc0e0((int *)param_1);
    uVar8 = FUN_005fc9c0(param_1);
    uVar9 = FUN_005fc9c0((uint *)this);
    if (param_3 < 0) {
      param_5 = param_5 - param_3;
      if (param_7 != 0) {
        param_7 = param_7 - param_3;
      }
      param_3 = 0;
    }
    if (param_2 < 0) {
      param_4 = param_4 - param_2;
      if (param_6 != 0) {
        param_6 = param_6 - param_2;
      }
      param_2 = 0;
    }
    if (local_30 < 0) {
      local_30 = -local_30;
      local_28 = 1;
    }
    else {
      local_28 = -1;
    }
    local_2c = FUN_005fc0f0((int *)this);
    if (local_2c < 0) {
      local_2c = -local_2c;
      local_24 = 1;
    }
    else {
      local_24 = -1;
    }
    iVar10 = FUN_005fc9e0(param_1,param_3);
    iVar11 = FUN_005fc9e0(this,param_5);
    if (param_6 == 0) {
      param_6 = iVar3 - param_4;
    }
    if (param_7 == 0) {
      param_7 = local_2c - param_5;
    }
    if (local_30 - param_3 < param_7) {
      param_7 = local_30 - param_3;
    }
    if (iVar7 - param_2 < param_6) {
      param_6 = iVar7 - param_2;
    }
    if (0 < param_7) {
      pbVar13 = (byte *)(uVar8 * iVar10 + iVar4 + param_2);
      iVar3 = uVar9 * iVar11 + iVar6 + param_4;
      param_3 = param_7;
      do {
        if (0 < param_6) {
          pbVar12 = pbVar13;
          param_7 = param_6;
          do {
            if (pbVar12[iVar3 - (int)pbVar13] != bVar1) {
              *pbVar12 = *(byte *)((uint)*pbVar12 + *(int *)((int)this + 0x24));
            }
            pbVar12 = pbVar12 + 1;
            param_7 = param_7 + -1;
          } while (param_7 != 0);
        }
        iVar3 = iVar3 + uVar9 * local_24;
        pbVar13 = pbVar13 + uVar8 * local_28;
        param_3 = param_3 + -1;
      } while (param_3 != 0);
    }
    FUN_005fd170(param_1);
  }
  return 1;
}

