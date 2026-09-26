
uint __cdecl FUN_004a1f60(int param_1,int param_2)

{
  bool bVar1;
  short sVar2;
  short sVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  undefined4 *puVar7;
  uint uVar8;
  undefined4 *this;
  uint uVar9;
  int *piVar10;
  undefined1 *puVar11;
  undefined1 local_44 [4];
  uint local_40;
  undefined4 local_3c [2];
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  iVar6 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637830;
  local_c = ExceptionList;
  if (param_1 == 0) {
    return 0;
  }
  if (param_2 == 1) {
    uVar8 = 1;
  }
  else if (param_2 == 2) {
    uVar8 = 2;
  }
  else {
    uVar8 = 0;
  }
  ExceptionList = &local_c;
  FUN_004f4340(local_34);
  local_4 = 0;
  FUN_004f4340(local_3c);
  local_4._0_1_ = 1;
  FUN_00536da0(local_2c,iVar6,3);
  local_4._0_1_ = 2;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) {
      sVar2 = FUN_005f50e0((int)local_34);
      sVar3 = FUN_005f50e0((int)local_3c);
      if (uVar8 == 3) {
        uVar9 = 3;
      }
      else {
        uVar9 = 2 - (uVar8 != 1);
      }
      if (sVar3 == 0) {
        sVar3 = 0;
        if (sVar2 != 0) {
          uVar9 = uVar8;
          sVar3 = sVar2;
        }
        if (sVar3 == 0) {
          uVar9 = 3;
        }
      }
      local_4._0_1_ = 1;
      FUN_00536ea0(local_2c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_004f4380(local_3c);
      local_4 = 0xffffffff;
      FUN_004f4380(local_34);
      ExceptionList = local_c;
      return uVar9;
    }
    piVar10 = &param_1;
    pvVar4 = (void *)FUN_0052bed0((int)local_2c);
    puVar5 = FUN_0042d170(pvVar4,piVar10);
    local_4 = CONCAT31(local_4._1_3_,3);
    iVar6 = FUN_004ece60(puVar5);
    if (iVar6 == 0) {
LAB_004a2040:
      bVar1 = false;
    }
    else {
      iVar6 = FUN_0052bed0((int)local_2c);
      local_40 = *(uint *)(iVar6 + 0x78) >> 8 & 1;
      if (local_40 != 0) goto LAB_004a2040;
      bVar1 = true;
    }
    local_4._0_1_ = 2;
    FUN_00619730();
    if (bVar1) {
      this = (undefined4 *)0x0;
      iVar6 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar6 + 0x24) >> 6 & 3) == uVar8) {
        this = local_34;
      }
      else {
        iVar6 = FUN_0052bed0((int)local_2c);
        if (uVar8 == 3) {
          uVar9 = 3;
        }
        else {
          uVar9 = 2 - (uVar8 != 1);
        }
        if ((*(uint *)(iVar6 + 0x24) >> 6 & 3) == uVar9) {
          this = local_3c;
        }
      }
      if (this != (undefined4 *)0x0) {
        piVar10 = &param_2;
        pvVar4 = (void *)FUN_0052bed0((int)local_2c);
        puVar5 = FUN_0042d170(pvVar4,piVar10);
        local_4._0_1_ = 4;
        pvVar4 = FUN_004f5940(this,puVar5);
        local_4._0_1_ = 2;
        FUN_00619730();
        if (pvVar4 == (void *)0x0) {
          puVar11 = local_44;
          pvVar4 = (void *)FUN_0052bed0((int)local_2c);
          puVar7 = FUN_0042d170(pvVar4,puVar11);
          local_4._0_1_ = 5;
          FUN_004f44b0(this,puVar7,0);
          local_4._0_1_ = 2;
          FUN_00619730();
        }
      }
    }
    FUN_005130d0((int)local_2c);
  } while( true );
}

