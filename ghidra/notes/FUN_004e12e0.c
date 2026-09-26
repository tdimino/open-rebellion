
void __fastcall FUN_004e12e0(int param_1)

{
  bool bVar1;
  int *this;
  uint *puVar2;
  uint local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063db98;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  for (this = FUN_004f2d10(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x50)); this != (int *)0x0;
      this = (int *)this[7]) {
    puVar2 = FUN_004025b0(this,&local_18);
    local_14 = 0x90;
    local_10 = 0x98;
    local_4 = 0;
    if ((*puVar2 >> 0x18 < 0x90) || (0x97 < *puVar2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) break;
  }
  puVar2 = FUN_004025b0(this,&local_18);
  local_14 = 0x90;
  local_10 = 0x98;
  bVar1 = true;
  local_4 = 1;
  if ((*puVar2 >> 0x18 < 0x90) || (0x97 < *puVar2 >> 0x18)) {
    bVar1 = false;
  }
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    puVar2 = FUN_004025b0(this,&local_18);
    local_4 = 2;
    FUN_004f26d0((void *)(param_1 + 0x54),puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(undefined4 *)(param_1 + 0x60) = *(undefined4 *)(param_1 + 100);
    *(undefined4 *)(param_1 + 0x6c) = 0x1000;
    *(undefined4 *)(param_1 + 0x70) = 0x1000;
    *(undefined4 *)(param_1 + 0x74) = 1;
  }
  ExceptionList = local_c;
  return;
}

