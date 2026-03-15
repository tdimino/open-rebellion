
/* WARNING: Type propagation algorithm not settling */

void __fastcall FUN_0047e3f0(int *param_1)

{
  int *this;
  bool bVar1;
  ushort uVar2;
  int iVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint local_20 [5];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633c50;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(local_20);
  bVar1 = false;
  local_4 = 0;
  param_1[8] = param_1[8] & 0xfffffffd;
  uVar2 = FUN_005f50e0((int)(param_1 + 0x11));
  iVar3 = (**(code **)(*param_1 + 0x44))();
  if ((int)(uint)uVar2 < iVar3) {
    pvVar4 = FUN_00403460((void *)param_1[0x19],0x10800002,2,0x7400051,0,1,0x33,10000,(void *)0x1);
    this = param_1 + 0x15;
    FUN_00435790(this,(int)pvVar4);
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((0x8f < (uint)param_1[0xe] >> 0x18) && ((uint)param_1[0xe] >> 0x18 < 0x98)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puVar5 = FUN_00403750((void *)param_1[0x19],param_1 + 0xe,0x10800002,2,0x7400051,0,2);
      FUN_00435790(this,(int)puVar5);
    }
    puVar5 = (undefined4 *)FUN_004357b0(this,local_20 + 1);
    local_4._0_1_ = 1;
    FUN_004f26d0(local_20,puVar5);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    do {
      local_20[3] = 0x30;
      local_20[4] = 0x40;
      local_4 = CONCAT31(local_4._1_3_,2);
      if ((local_20[0] >> 0x18 < 0x30) || (0x3f < local_20[0] >> 0x18)) {
LAB_0047e541:
        bVar1 = false;
      }
      else {
        uVar2 = FUN_005f50e0((int)(param_1 + 0x11));
        iVar3 = (**(code **)(*param_1 + 0x44))();
        if (iVar3 <= (int)(uint)uVar2) goto LAB_0047e541;
        bVar1 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (!bVar1) goto LAB_0047e59f;
      iVar3 = FUN_0047ba90(param_1,local_20);
      if (iVar3 != 0) {
        FUN_0047b730(param_1,local_20);
      }
      puVar5 = (undefined4 *)FUN_004357b0(this,local_20 + 1);
      local_4._0_1_ = 3;
      FUN_004f26d0(local_20,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    } while( true );
  }
LAB_0047e5a6:
  uVar2 = FUN_005f50e0((int)(param_1 + 0x11));
  iVar3 = (**(code **)(*param_1 + 0x44))();
  if (iVar3 <= (int)(uint)uVar2) {
    param_1[8] = param_1[8] | 2;
    uVar2 = FUN_005f50e0((int)(param_1 + 0x11));
    iVar3 = (**(code **)(*param_1 + 0x44))();
    if (iVar3 < (int)(uint)uVar2) {
      FUN_0047b9e0(param_1);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
LAB_0047e59f:
  FUN_005f58b0(this);
  goto LAB_0047e5a6;
}

