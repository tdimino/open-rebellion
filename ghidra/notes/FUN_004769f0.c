
void __thiscall FUN_004769f0(void *this,int param_1)

{
  void *pvVar1;
  uint *puVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  int *piVar6;
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632f70;
  local_c = ExceptionList;
  if (*(int *)((int)this + 0x44) != param_1) {
    ExceptionList = &local_c;
    *(int *)((int)this + 0x44) = param_1;
    for (pvVar1 = (void *)thunk_FUN_005f5060((int)this + 0x24); pvVar1 != (void *)0x0;
        pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
      puVar2 = FUN_00403040(pvVar1,local_14);
      local_4 = 0;
      pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x58) + 0x58),puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x30) == *(int *)((int)this + 0x18))) {
        *(int *)((int)pvVar3 + 0x34) = param_1;
      }
    }
    for (pvVar1 = (void *)thunk_FUN_005f5060((int)this + 0x2c); pvVar1 != (void *)0x0;
        pvVar1 = *(void **)((int)pvVar1 + 0x10)) {
      puVar4 = FUN_00403040(pvVar1,local_14);
      iVar5 = *(int *)((int)this + 0x58);
      local_10 = &stack0xffffffd8;
      local_4 = 1;
      FUN_004f26d0(&stack0xffffffd8,puVar4);
      pvVar3 = FUN_00419e40((void *)(iVar5 + 0x78));
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x30) == *(int *)((int)this + 0x18))) {
        *(int *)((int)pvVar3 + 0x34) = param_1;
      }
    }
    for (iVar5 = thunk_FUN_005f5060((int)this + 0xb0); iVar5 != 0; iVar5 = *(int *)(iVar5 + 0x10)) {
      piVar6 = (int *)FUN_005f5500((void *)(*(int *)((int)this + 0x58) + 0xa8),
                                   *(uint *)(iVar5 + 0x18));
      if (piVar6 != (int *)0x0) {
        (**(code **)(*piVar6 + 0x1c))();
      }
    }
    for (iVar5 = thunk_FUN_005f5060((int)this + 0xd4); iVar5 != 0; iVar5 = *(int *)(iVar5 + 0x10)) {
      piVar6 = (int *)FUN_005f5500((void *)(*(int *)((int)this + 0x58) + 0xec),
                                   *(uint *)(iVar5 + 0x18));
      if (piVar6 != (int *)0x0) {
        (**(code **)(*piVar6 + 0x20))();
      }
    }
  }
  ExceptionList = local_c;
  return;
}

