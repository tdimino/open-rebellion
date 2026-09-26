
void __fastcall FUN_0047b110(void *param_1)

{
  void *pvVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  undefined1 local_14 [4];
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006336a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xfffffeff;
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x3c);
  while (pvVar2 != (void *)0x0) {
    puVar3 = FUN_00403040(pvVar2,local_14);
    local_4 = 0;
    iVar4 = FUN_0047ba40(param_1,puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 == 0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,local_10);
      local_4 = 1;
      FUN_0047b870(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
    else {
      *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 0x100;
      pvVar2 = *(void **)((int)pvVar2 + 0x10);
    }
  }
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x44);
  while (pvVar2 != (void *)0x0) {
    puVar3 = FUN_00403040(pvVar2,local_10);
    local_4 = 2;
    iVar4 = FUN_0047ba40(param_1,puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 == 0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,local_14);
      local_4 = 3;
      FUN_0047b8e0(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
    else {
      *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 0x100;
      pvVar2 = *(void **)((int)pvVar2 + 0x10);
    }
  }
  ExceptionList = local_c;
  return;
}

