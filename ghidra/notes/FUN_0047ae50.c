
void __fastcall FUN_0047ae50(void *param_1)

{
  bool bVar1;
  void *pvVar2;
  uint *puVar3;
  void *pvVar4;
  undefined1 local_14 [4];
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633650;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0x1fffffff;
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x3c);
  while (pvVar2 != (void *)0x0) {
    bVar1 = false;
    puVar3 = FUN_00403040(pvVar2,local_14);
    local_4 = 0;
    pvVar4 = FUN_004f5940(*(void **)((int)param_1 + 100),puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (pvVar4 != (void *)0x0) {
      *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) & 0xffffbfff;
      FUN_004021b0((int)pvVar4);
      if (((*(byte *)((int)param_1 + 0x20) & 0x80) == 0) &&
         ((*(byte *)((int)pvVar4 + 0x30) & 0x50) == 0)) {
        bVar1 = true;
      }
    }
    if (bVar1) {
      pvVar2 = *(void **)((int)pvVar2 + 0x10);
    }
    else {
      pvVar4 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,local_10);
      local_4 = 1;
      FUN_0047b870(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar4;
    }
  }
  if ((*(byte *)((int)param_1 + 0x20) & 0x80) != 0) {
    FUN_004ece80((undefined4 *)((int)param_1 + 0x38));
  }
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xffffff7f;
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x44);
  while (pvVar2 != (void *)0x0) {
    bVar1 = false;
    puVar3 = FUN_00403040(pvVar2,local_10);
    local_4 = 2;
    pvVar4 = FUN_004f5940(*(void **)((int)param_1 + 100),puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (pvVar4 != (void *)0x0) {
      *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) & 0xffffbfff;
      FUN_004021b0((int)pvVar4);
      if ((*(byte *)((int)pvVar4 + 0x30) & 0x50) == 0) {
        bVar1 = true;
      }
    }
    if (bVar1) {
      pvVar2 = *(void **)((int)pvVar2 + 0x10);
    }
    else {
      pvVar4 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,local_14);
      local_4 = 3;
      FUN_0047b8e0(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar4;
    }
  }
  ExceptionList = local_c;
  return;
}

