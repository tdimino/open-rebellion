
uint __fastcall FUN_005775a0(int *param_1)

{
  uint uVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  uint uVar4;
  uint local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064cd10;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_004f9700(param_1);
  FUN_004fcd00(local_2c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar4 = 0;
    if (uVar1 == 0) goto LAB_00577716;
    puVar2 = &local_58;
    this = (void *)FUN_0052bed0((int)local_2c);
    puVar2 = FUN_004025b0(this,puVar2);
    uVar1 = *puVar2;
    FUN_00619730();
    local_54 = 0x30;
    local_50 = 0x40;
    if ((uVar1 >> 0x18 < 0x30) || (0x3f < uVar1 >> 0x18)) {
      uVar1 = 0;
    }
    else {
      uVar1 = 1;
    }
    FUN_00619730();
    FUN_005130d0((int)local_2c);
  }
  uVar4 = 0;
  if (uVar1 != 0) {
    local_58 = 0;
    iVar3 = FUN_005770b0(param_1,&local_58);
    uVar4 = 0;
    if ((iVar3 != 0) && (uVar4 = 0, local_58 == 0)) {
      uVar4 = 1;
      FUN_00536da0(local_4c,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00513120((int)local_4c);
      while ((local_30 != 0 && (uVar4 != 0))) {
        iVar3 = FUN_0052bed0((int)local_4c);
        local_58 = *(uint *)(iVar3 + 0x78) >> 7 & 1;
        if (local_58 == 0) {
          iVar3 = FUN_0052bed0((int)local_4c);
          uVar4 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
        }
        FUN_005130d0((int)local_4c);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00536ea0(local_4c);
    }
  }
LAB_00577716:
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar4;
}

