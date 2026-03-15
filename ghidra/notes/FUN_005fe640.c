
undefined4 __thiscall
FUN_005fe640(void *this,uint *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            int param_7)

{
  byte bVar1;
  byte bVar2;
  bool bVar3;
  int iVar4;
  undefined3 extraout_var;
  int iVar5;
  undefined4 uVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  byte *pbVar14;
  byte *pbVar15;
  int local_20;
  int local_18;
  int local_14;
  
  iVar4 = FUN_005fc0e0((int *)this);
  bVar3 = FUN_005fc040((int *)this);
  if (CONCAT31(extraout_var,bVar3) != 0) {
    if (*(int *)((int)this + 0x24) == 0) {
      return 0;
    }
    bVar1 = *(byte *)((int)this + 8);
    iVar5 = FUN_005fc110((int *)param_1);
    if (iVar5 != 0) {
      return 0;
    }
    iVar5 = FUN_005fc110((int *)this);
    if (iVar5 == 1) {
      uVar6 = FUN_005897b0();
      return uVar6;
    }
    iVar5 = FUN_00583c40((int)param_1);
    iVar7 = FUN_00583c40((int)this);
    local_20 = FUN_005fc0f0((int *)param_1);
    iVar8 = FUN_005fc0e0((int *)param_1);
    uVar9 = FUN_005fc9c0(param_1);
    uVar10 = FUN_005fc9c0((uint *)this);
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
    if (local_20 < 0) {
      local_20 = -local_20;
      local_18 = 1;
    }
    else {
      local_18 = -1;
    }
    iVar11 = FUN_005fc0f0((int *)this);
    if (iVar11 < 0) {
      local_14 = 1;
      iVar11 = -iVar11;
    }
    else {
      local_14 = -1;
    }
    iVar12 = FUN_005fc9e0(param_1,param_3);
    iVar13 = FUN_005fc9e0(this,param_5);
    if (param_6 == 0) {
      param_6 = iVar4 - param_4;
    }
    if (param_7 == 0) {
      param_7 = iVar11 - param_5;
    }
    if (local_20 - param_3 < param_7) {
      param_7 = local_20 - param_3;
    }
    if (iVar8 - param_2 < param_6) {
      param_6 = iVar8 - param_2;
    }
    if (0 < param_7) {
      pbVar15 = (byte *)(uVar9 * iVar12 + iVar5 + param_2);
      iVar4 = uVar10 * iVar13 + iVar7 + param_4;
      do {
        if (0 < param_6) {
          pbVar14 = pbVar15;
          iVar5 = param_6;
          do {
            if (pbVar14[iVar4 - (int)pbVar15] == bVar1) {
              bVar2 = *(byte *)(*pbVar14 + 0x100 + *(int *)((int)this + 0x24));
            }
            else {
              bVar2 = *(byte *)(*(int *)((int)this + 0x24) + (uint)pbVar14[iVar4 - (int)pbVar15]);
            }
            *pbVar14 = bVar2;
            pbVar14 = pbVar14 + 1;
            iVar5 = iVar5 + -1;
          } while (iVar5 != 0);
        }
        iVar4 = iVar4 + uVar10 * local_14;
        pbVar15 = pbVar15 + uVar9 * local_18;
        param_7 = param_7 + -1;
      } while (param_7 != 0);
    }
    FUN_005fd170(param_1);
  }
  return 1;
}

