
void __fastcall FUN_004be4c0(void *param_1)

{
  void *pvVar1;
  void *pvVar2;
  uint *puVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639ef0;
  local_c = ExceptionList;
  if ((*(uint *)((int)param_1 + 0x20) & 0xe0000000) == 0) {
    ExceptionList = &local_c;
    local_10 = param_1;
    pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x3c);
    while (pvVar2 != (void *)0x0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,&local_10);
      local_4 = 0;
      FUN_0047b870(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
    pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x44);
    while (pvVar2 != (void *)0x0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,&local_10);
      local_4 = 1;
      FUN_0047b8e0(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
  }
  ExceptionList = local_c;
  return;
}

