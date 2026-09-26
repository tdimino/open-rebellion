
/* WARNING: Type propagation algorithm not settling */

void __fastcall FUN_004808c0(void *param_1)

{
  void *this;
  bool bVar1;
  ushort uVar2;
  void *pvVar3;
  uint uVar4;
  undefined4 *puVar5;
  int iVar6;
  uint local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634030;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_20);
  bVar1 = false;
  local_4 = 0;
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xfffffffe;
  uVar2 = FUN_005f50e0((int)param_1 + 0x3c);
  if (uVar2 < 2) {
    pvVar3 = FUN_00403460(*(void **)((int)param_1 + 100),0x10800002,8,0x7400051,0,1,0x33,10000,
                          (void *)0x1);
    this = (void *)((int)param_1 + 0x54);
    FUN_00435790(this,(int)pvVar3);
    uVar4 = *(uint *)((int)param_1 + 0x38) >> 0x18;
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((0x8f < uVar4) && (uVar4 < 0x98)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      puVar5 = FUN_00403750(*(void **)((int)param_1 + 100),(undefined4 *)((int)param_1 + 0x38),
                            0x10800002,8,0x7400051,0,2);
      FUN_00435790(this,(int)puVar5);
    }
    puVar5 = FUN_004357b0(this,local_20 + 1);
    local_4._0_1_ = 1;
    FUN_004f26d0(local_20,puVar5);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    do {
      local_20[3] = 0x30;
      local_20[4] = 0x40;
      local_4 = CONCAT31(local_4._1_3_,2);
      if ((local_20[0] >> 0x18 < 0x30) || (0x3f < local_20[0] >> 0x18)) {
LAB_004809f4:
        bVar1 = false;
      }
      else {
        uVar2 = FUN_005f50e0((int)param_1 + 0x3c);
        if (1 < uVar2) goto LAB_004809f4;
        bVar1 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (!bVar1) goto LAB_00480a52;
      iVar6 = FUN_0047ba90(param_1,local_20);
      if (iVar6 != 0) {
        FUN_0047b610(param_1,local_20);
      }
      puVar5 = FUN_004357b0(this,local_20 + 1);
      local_4._0_1_ = 3;
      FUN_004f26d0(local_20,puVar5);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    } while( true );
  }
LAB_00480a59:
  uVar2 = FUN_005f50e0((int)param_1 + 0x3c);
  if (1 < uVar2) {
    *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 1;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
LAB_00480a52:
  FUN_005f58b0(this);
  goto LAB_00480a59;
}

