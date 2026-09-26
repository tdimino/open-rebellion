
uint __fastcall FUN_004ff040(int *param_1)

{
  uint uVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  uint uVar5;
  uint local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640648;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_004f9700(param_1);
  FUN_004fcd00(local_2c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    uVar5 = 0;
    if (uVar1 == 0) goto LAB_004ff152;
    puVar2 = &local_38;
    this = (void *)FUN_0052bed0((int)local_2c);
    puVar2 = FUN_004025b0(this,puVar2);
    uVar1 = *puVar2;
    FUN_00619730();
    local_34 = 0x14;
    local_30 = 0x1c;
    if ((uVar1 >> 0x18 < 0x14) || (0x1b < uVar1 >> 0x18)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    FUN_00619730();
    uVar1 = 0;
    if (bVar4) {
      iVar3 = FUN_0052bed0((int)local_2c);
      uVar1 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
    }
    FUN_005130d0((int)local_2c);
  }
  uVar5 = 0;
  if (uVar1 != 0) {
    local_38 = 0;
    iVar3 = FUN_004fd840(param_1,(int *)&local_38);
    uVar5 = 0;
    if ((iVar3 != 0) && (uVar5 = 0, (int)local_38 < 2)) {
      uVar5 = 1;
    }
  }
LAB_004ff152:
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar5;
}

