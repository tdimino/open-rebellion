
int __fastcall FUN_004820e0(int param_1)

{
  void *this;
  uint *puVar1;
  void *pvVar2;
  int local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634258;
  local_c = ExceptionList;
  local_14 = 0;
  if ((*(byte *)(param_1 + 0x20) & 0x20) == 0) {
    ExceptionList = &local_c;
    for (this = (void *)thunk_FUN_005f5060(param_1 + 0x3c); this != (void *)0x0;
        this = *(void **)((int)this + 0x10)) {
      puVar1 = (uint *)FUN_00403040(this,local_10);
      local_4 = 0;
      pvVar2 = FUN_004f5940(*(void **)(param_1 + 100),puVar1);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (pvVar2 != (void *)0x0) {
        local_14 = local_14 + (*(int *)((int)pvVar2 + 0x54) + *(int *)((int)pvVar2 + 0x44)) / 2;
      }
    }
  }
  ExceptionList = local_c;
  return local_14;
}

