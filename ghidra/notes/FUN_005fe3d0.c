
undefined4 __thiscall
FUN_005fe3d0(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,uint param_6,
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
  int iVar12;
  byte *pbVar13;
  uint uVar14;
  byte *pbVar15;
  int local_24;
  int local_20;
  int local_1c;
  
  iVar3 = FUN_005fc0e0((int *)this);
  bVar1 = *(byte *)((int)this + 8);
  bVar2 = FUN_005fc040((int *)this);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    if (*(int *)((int)this + 0x24) == 0) {
      return 0;
    }
    iVar4 = FUN_005fc110((int *)param_1);
    if (iVar4 != 0) {
      return 0;
    }
    iVar4 = FUN_005fc110((int *)this);
    if (iVar4 == 1) {
      uVar5 = FUN_005fe8e0(this,param_1,param_2,param_3,bVar1,param_4,param_5,param_6,param_7);
      return uVar5;
    }
    iVar4 = FUN_00583c40((int)param_1);
    iVar6 = FUN_00583c40((int)this);
    local_24 = FUN_005fc0f0((int *)param_1);
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
    if (local_24 < 0) {
      local_24 = -local_24;
      local_1c = 1;
    }
    else {
      local_1c = -1;
    }
    iVar10 = FUN_005fc0f0((int *)this);
    if (iVar10 < 0) {
      local_20 = 1;
      iVar10 = -iVar10;
    }
    else {
      local_20 = -1;
    }
    iVar11 = FUN_005fc9e0(param_1,param_3);
    iVar12 = FUN_005fc9e0(this,param_5);
    if (param_6 == 0) {
      param_6 = iVar3 - param_4;
    }
    if (param_7 == 0) {
      param_7 = iVar10 - param_5;
    }
    if (local_24 - param_3 < param_7) {
      param_7 = local_24 - param_3;
    }
    if (iVar7 - param_2 < (int)param_6) {
      param_6 = iVar7 - param_2;
    }
    if (0 < param_7) {
      pbVar15 = (byte *)(uVar9 * iVar12 + iVar6 + param_4);
      iVar3 = uVar8 * iVar11 + iVar4 + param_2;
      do {
        if (0 < (int)param_6) {
          pbVar13 = pbVar15;
          uVar14 = param_6;
          do {
            if (*pbVar13 != bVar1) {
              pbVar13[iVar3 - (int)pbVar15] = *(byte *)(*(int *)((int)this + 0x24) + (uint)*pbVar13)
              ;
            }
            pbVar13 = pbVar13 + 1;
            uVar14 = uVar14 - 1;
          } while (uVar14 != 0);
        }
        iVar3 = iVar3 + uVar8 * local_1c;
        pbVar15 = pbVar15 + uVar9 * local_20;
        param_7 = param_7 + -1;
      } while (param_7 != 0);
    }
    FUN_005fd170(param_1);
  }
  return 1;
}

