
void __fastcall FUN_0043dd30(int param_1)

{
  bool bVar1;
  int *this;
  uint *puVar2;
  uint local_24;
  uint local_20;
  void *pvStack_1c;
  void *pvStack_18;
  uint auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062e178;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = FUN_004f2ec0(*(int *)(param_1 + 0x18),(uint *)(param_1 + 0x20));
  FUN_004ece30(&local_20);
  local_4 = 0;
  FUN_004ece30(&local_24);
  local_4._0_1_ = 1;
  if (this != (int *)0x0) {
    FUN_004f6b70(this,&local_20);
    (**(code **)(*this + 0xc))(&local_24);
    auStack_14[0] = 0x14;
    auStack_14[1] = 0x1c;
    if ((local_20 >> 0x18 < 0x14) || (0x1b < local_20 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0((void *)(param_1 + 0x2c),&local_20);
      pvStack_1c = (void *)0x0;
      pvStack_18 = (void *)0x0;
      FUN_004f6b50(this,(int *)&pvStack_1c);
      if (pvStack_1c != (void *)0x0) {
        FUN_004f6b50(pvStack_1c,(int *)&pvStack_18);
      }
      if (pvStack_18 != (void *)0x0) {
        puVar2 = FUN_004025b0(pvStack_18,auStack_14);
        local_4._0_1_ = 2;
        FUN_004f26d0((void *)(param_1 + 0x30),puVar2);
        local_4._0_1_ = 1;
        FUN_00619730();
      }
    }
    auStack_14[0] = 0x90;
    auStack_14[1] = 0x98;
    if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0((void *)(param_1 + 0x34),&local_24);
    }
    auStack_14[0] = 0x98;
    auStack_14[1] = 0xa0;
    if ((local_24 >> 0x18 < 0x98) || (0x9f < local_24 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0((void *)(param_1 + 0x38),&local_24);
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

