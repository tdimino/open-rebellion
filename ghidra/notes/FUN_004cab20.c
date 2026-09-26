
int * __fastcall FUN_004cab20(int param_1)

{
  uint *this;
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  int iVar5;
  int *piVar6;
  void *local_24;
  int *local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b273;
  pvStack_c = ExceptionList;
  local_20 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_1c);
  local_4 = 0;
  pvVar2 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),5,0x402,0xe,&DAT_00000002);
  pvVar4 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,&local_24);
  this = (uint *)(param_1 + 0x54);
  local_4._0_1_ = 1;
  FUN_004f26d0(this,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  pvVar2 = FUN_0041ac80(*(void **)(param_1 + 0x2c),this,0x24000,0x2800,0xb,&DAT_00000002);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,&local_24);
  local_4._0_1_ = 2;
  FUN_004f26d0(this,puVar3);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  local_14 = 0x14;
  local_10 = 0x1c;
  if ((*this >> 0x18 < 0x14) || (0x1b < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    local_24 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 3;
    if (local_24 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_004f5b10(local_24,this,0);
    }
    local_4._0_1_ = 0;
    if (pvVar4 != (void *)0x0) {
      FUN_004f57b0(local_1c,pvVar4);
    }
  }
  iVar5 = thunk_FUN_005f5060((int)local_1c);
  piVar6 = local_20;
  if ((iVar5 != 0) && (piVar6 = (int *)FUN_004f5cd0(0x200), piVar6 != (int *)0x0)) {
    piVar6[8] = *(int *)(param_1 + 0x30);
    (**(code **)(*piVar6 + 0x24))(local_1c);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_1c);
  ExceptionList = pvStack_c;
  return piVar6;
}

