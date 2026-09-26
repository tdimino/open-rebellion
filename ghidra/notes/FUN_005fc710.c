
undefined4 __thiscall
FUN_005fc710(void *this,uint *param_1,int param_2,int param_3,uint param_4,int param_5,byte *param_6
            ,uint param_7,uint param_8)

{
  uint *this_00;
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  uint uVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  uint *puVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  byte *pbVar12;
  byte *pbVar13;
  int local_20;
  int local_1c;
  int local_18;
  
  this_00 = param_1;
  iVar2 = FUN_005fc0e0(this);
  bVar1 = FUN_005fc040(this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    uVar3 = FUN_005fc100(param_1);
    if (uVar3 == 0x18) {
      FUN_0060e650(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8);
      return 1;
    }
    uVar3 = FUN_005fc100(this);
    if (uVar3 == 0x18) {
      return 0;
    }
    iVar4 = FUN_005fc110((int *)param_1);
    if (iVar4 != 0) {
      return 0;
    }
    iVar4 = FUN_005fc110(this);
    if (iVar4 == 1) {
      uVar5 = FUN_005fd7c0(this,param_1,param_2,param_3,(byte)param_4,param_5,(int)param_6,param_7,
                           param_8);
      return uVar5;
    }
    iVar4 = FUN_00583c40((int)param_1);
    iVar6 = FUN_00583c40((int)this);
    puVar7 = (uint *)FUN_005fc0f0((int *)param_1);
    iVar8 = FUN_005fc0e0((int *)param_1);
    uVar3 = FUN_005fc9c0(param_1);
    uVar9 = FUN_005fc9c0(this);
    if (param_3 < 0) {
      param_6 = param_6 + -param_3;
      if (param_8 != 0) {
        param_8 = param_8 - param_3;
      }
      param_3 = 0;
    }
    if (param_2 < 0) {
      param_5 = param_5 - param_2;
      if (param_7 != 0) {
        param_7 = param_7 - param_2;
      }
      param_2 = 0;
    }
    if ((int)puVar7 < 0) {
      local_18 = 1;
      param_1 = (uint *)-(int)puVar7;
    }
    else {
      local_18 = -1;
      param_1 = puVar7;
    }
    local_20 = FUN_005fc0f0(this);
    if (local_20 < 0) {
      local_20 = -local_20;
      local_1c = 1;
    }
    else {
      local_1c = -1;
    }
    iVar10 = FUN_005fc9e0(this_00,param_3);
    iVar11 = FUN_005fc9e0(this,(int)param_6);
    if (param_7 == 0) {
      param_7 = iVar2 - param_5;
    }
    if (param_8 == 0) {
      param_8 = local_20 - (int)param_6;
    }
    param_6 = (byte *)param_8;
    if ((int)param_1 - param_3 < (int)param_8) {
      param_6 = (byte *)((int)param_1 - param_3);
    }
    if (iVar8 - param_2 < (int)param_7) {
      param_7 = iVar8 - param_2;
    }
    if (0 < (int)param_6) {
      pbVar13 = (byte *)(uVar9 * iVar11 + iVar6 + param_5);
      iVar2 = uVar3 * iVar10 + iVar4 + param_2;
      do {
        if (0 < (int)param_7) {
          pbVar12 = pbVar13;
          param_8 = param_7;
          do {
            if (*pbVar12 != (byte)param_4) {
              pbVar12[iVar2 - (int)pbVar13] = *pbVar12;
            }
            pbVar12 = pbVar12 + 1;
            param_8 = param_8 - 1;
          } while (param_8 != 0);
        }
        iVar2 = iVar2 + uVar3 * local_18;
        pbVar13 = pbVar13 + uVar9 * local_1c;
        param_6 = param_6 + -1;
      } while (param_6 != (byte *)0x0);
    }
  }
  return 1;
}

