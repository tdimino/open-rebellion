
void __fastcall FUN_0047aff0(int *param_1)

{
  void *pvVar1;
  void *pvVar2;
  uint *puVar3;
  int *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633670;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = param_1;
  if (((param_1[8] & 0x10000000U) != 0) &&
     (ExceptionList = &pvStack_c, (param_1[8] & 0xe0000000U) == 0)) {
    ExceptionList = &pvStack_c;
    pvVar2 = (void *)thunk_FUN_005f5060((int)(param_1 + 0xf));
    while (pvVar2 != (void *)0x0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,&local_10);
      local_4 = 0;
      FUN_0047b870(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
    pvVar2 = (void *)thunk_FUN_005f5060((int)(param_1 + 0x11));
    while (pvVar2 != (void *)0x0) {
      pvVar1 = *(void **)((int)pvVar2 + 0x10);
      puVar3 = FUN_00403040(pvVar2,&local_10);
      local_4 = 1;
      FUN_0047b8e0(param_1,puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar2 = pvVar1;
    }
    FUN_0041b530((void *)param_1[0x1a],param_1[6]);
  }
  (**(code **)(*param_1 + 0x38))();
  ExceptionList = pvStack_c;
  return;
}

